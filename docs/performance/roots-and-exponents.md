# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 131 ns | 139 ns | 159 ns | 255 ns | 300 ns |
| D38 | 238 ns | 240 ns | 459 ns | 534 ns | 623 ns |
| D57 | 340 ns | 592 ns | 644 ns | 1.05 µs | 1.17 µs |
| D76 | 444 ns | 439 ns | 1.1 µs | 1.34 µs | 1.9 µs |
| D115 | 223 ns | 680 ns | 2.41 µs | 3.51 µs | 4.44 µs |
| D153 | 245 ns | 749 ns | 3.5 µs | 5.14 µs | 6.82 µs |
| D230 | 187 ns | 1.42 µs | 7.83 µs | 8.62 µs | 14.7 µs |
| D307 | 247 ns | 1.83 µs | 11.1 µs | 12.6 µs | 22.4 µs |
| D462 | 380 ns | 3.2 µs | 26.1 µs | 29.6 µs | 51.8 µs |
| D616 | 440 ns | 3.14 µs | 40.1 µs | 57.7 µs | 84.8 µs |
| D924 | 595 ns | 9.17 µs | 69.9 µs | 121 µs | 198 µs |
| D1232 | 1.15 µs | 16.1 µs | 151 µs | 195 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.2 88.2,191.1 124.4,183.4 160.5,177.6 196.7,192.6 232.9,190.5 269.1,196.4 305.3,190.4 341.5,181.0 377.6,177.8 413.8,171.3 450.0,157.0 450.0,33.1 413.8,45.1 377.6,63.6 341.5,74.3 305.3,92.4 269.1,101.7 232.9,118.3 196.7,127.6 160.5,146.0 124.4,156.6 88.2,170.3 52.0,186.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.2 88.2,191.1 124.4,183.4 160.5,177.6 196.7,192.6 232.9,190.5 269.1,196.4 305.3,190.4 341.5,181.0 377.6,177.8 413.8,171.3 450.0,157.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,202.9 88.2,191.0 124.4,171.4 160.5,177.9 196.7,168.4 232.9,166.3 269.1,152.4 305.3,146.8 341.5,134.8 377.6,135.2 413.8,111.9 450.0,99.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,199.9 88.2,176.9 124.4,169.6 160.5,157.9 196.7,140.9 232.9,132.8 269.1,115.3 305.3,107.8 341.5,89.2 377.6,79.8 413.8,67.8 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.6 88.2,173.6 124.4,159.0 160.5,153.6 196.7,132.7 232.9,124.5 269.1,113.2 305.3,105.0 341.5,86.4 377.6,71.9 413.8,55.9 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.1 88.2,170.3 124.4,156.6 160.5,146.0 196.7,127.6 232.9,118.3 269.1,101.7 305.3,92.4 341.5,74.3 377.6,63.6 413.8,45.1 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.79 µs | 3.51 µs | 2.98 µs | 3.98 µs |
| D38 | 1.87 ns | 2.78 µs | 4.16 µs | 4.68 µs | 3.95 µs |
| D57 | 2.81 ns | 3.86 µs | 4.93 µs | 6.44 µs | 9.27 µs |
| D76 | 3.43 ns | 3.46 µs | 7.64 µs | 9.39 µs | 11.4 µs |
| D115 | 11.8 ns | 5.22 µs | 13.4 µs | 18.1 µs | 20.6 µs |
| D153 | 16.2 ns | 5.92 µs | 15.9 µs | 22.8 µs | 34.9 µs |
| D230 | 23.9 ns | 13.8 µs | 22 µs | 36.8 µs | 84.2 µs |
| D307 | 43.9 ns | 16.2 µs | 34.8 µs | 61.7 µs | 122 µs |
| D462 | 111 ns | 23.2 µs | 85.8 µs | 147 µs | 249 µs |
| D616 | 118 ns | 22.8 µs | 142 µs | 269 µs | 444 µs |
| D924 | 214 ns | 81 µs | 227 µs | 556 µs | 985 µs |
| D1232 | 377 ns | 141 µs | 444 µs | 776 µs | 2.81 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,202.2 124.4,197.2 160.5,194.7 196.7,179.3 232.9,175.4 269.1,170.6 305.3,163.1 341.5,151.6 377.6,150.9 413.8,143.4 450.0,136.4 450.0,25.7 413.8,38.8 377.6,48.6 341.5,55.8 305.3,64.7 269.1,69.3 232.9,80.2 196.7,86.7 160.5,94.1 124.4,96.7 88.2,107.2 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,202.2 124.4,197.2 160.5,194.7 196.7,179.3 232.9,175.4 269.1,170.6 305.3,163.1 341.5,151.6 377.6,150.9 413.8,143.4 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.0 88.2,111.6 124.4,107.5 160.5,108.9 196.7,103.8 232.9,102.2 269.1,91.7 305.3,89.8 341.5,85.3 377.6,85.5 413.8,69.8 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,106.6 124.4,104.5 160.5,99.1 196.7,92.1 232.9,89.9 269.1,86.0 305.3,80.2 341.5,69.0 377.6,62.8 413.8,57.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,105.1 124.4,101.2 160.5,96.5 196.7,88.4 232.9,85.5 269.1,79.5 305.3,73.1 341.5,62.4 377.6,54.9 413.8,45.8 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,107.2 124.4,96.7 160.5,94.1 196.7,86.7 232.9,80.2 269.1,69.3 305.3,64.7 341.5,55.8 377.6,48.6 413.8,38.8 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 39.7 ns | 42.9 ns | 31.3 ns | 40.1 ns |
| D38 | 16 ns | 33.1 ns | 74.4 ns | 75.2 ns | 97.3 ns |
| D57 | 17.6 ns | 41.1 ns | 74.9 ns | 398 ns | 394 ns |
| D76 | 18.9 ns | 48.3 ns | 391 ns | 382 ns | 619 ns |
| D115 | 22.1 ns | 63.8 ns | 391 ns | 777 ns | 964 ns |
| D153 | 26.1 ns | 338 ns | 673 ns | 930 ns | 1.58 µs |
| D230 | 15.3 ns | 456 ns | 1.08 µs | 1.55 µs | 2.93 µs |
| D307 | 19.1 ns | 738 ns | 1.65 µs | 2.29 µs | 5.3 µs |
| D462 | 61.7 ns | 1.15 µs | 3.35 µs | 5.41 µs | 7.84 µs |
| D616 | 61.5 ns | 1.31 µs | 5.91 µs | 10.3 µs | 13.9 µs |
| D924 | 114 ns | 3.25 µs | 8.6 µs | 22.3 µs | 26.9 µs |
| D1232 | 100 ns | 6.05 µs | 20.4 µs | 20.1 µs | 48.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,199.9 124.4,197.8 160.5,196.1 196.7,192.7 232.9,189.2 269.1,200.7 305.3,196.0 341.5,170.5 377.6,170.6 413.8,157.2 450.0,160.0 450.0,25.6 413.8,38.5 377.6,52.8 341.5,65.3 305.3,73.8 269.1,86.7 232.9,100.1 196.7,110.8 160.5,120.4 124.4,130.2 88.2,160.6 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,199.9 124.4,197.8 160.5,196.1 196.7,192.7 232.9,189.2 269.1,200.7 305.3,196.0 341.5,170.5 377.6,170.6 413.8,157.2 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.1 88.2,184.0 124.4,179.3 160.5,175.8 196.7,169.8 232.9,133.6 269.1,127.0 305.3,116.6 341.5,106.9 377.6,104.1 413.8,84.4 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,166.4 124.4,166.3 160.5,130.4 196.7,130.4 232.9,118.6 269.1,108.3 305.3,99.2 341.5,83.7 377.6,71.4 413.8,63.3 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.2 88.2,166.2 124.4,130.0 160.5,130.9 196.7,115.5 232.9,111.6 269.1,100.5 305.3,92.0 341.5,73.3 377.6,59.4 413.8,42.6 450.0,44.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,160.6 124.4,130.2 160.5,120.4 196.7,110.8 232.9,100.1 269.1,86.7 305.3,73.8 341.5,65.3 377.6,52.8 413.8,38.5 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 292 ns | 399 ns | 332 ns | 410 ns |
| D38 | 2.18 ns | 328 ns | 409 ns | 412 ns | 377 ns |
| D57 | 279 ns | 466 ns | 510 ns | 515 ns | 629 ns |
| D76 | 286 ns | 294 ns | 511 ns | 618 ns | 616 ns |
| D115 | 279 ns | 395 ns | 628 ns | 965 ns | 878 ns |
| D153 | 299 ns | 441 ns | 762 ns | 1.23 µs | 1.29 µs |
| D230 | 317 ns | 722 ns | 1.02 µs | 1.25 µs | 1.91 µs |
| D307 | 385 ns | 802 ns | 1.12 µs | 1.23 µs | 11.1 µs |
| D462 | 1.19 µs | 2.81 µs | 3.66 µs | 3.55 µs | 5.45 µs |
| D616 | 1.3 µs | 984 ns | 2.06 µs | 2.91 µs | 3.97 µs |
| D924 | 2.22 µs | 2.04 µs | 2.56 µs | 3.64 µs | 5.06 µs |
| D1232 | 3.09 µs | 3.36 µs | 4.47 µs | 4.6 µs | 7.24 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,196.4 124.4,112.2 160.5,111.7 196.7,112.2 232.9,110.9 269.1,110.0 305.3,106.6 341.5,86.9 377.6,85.5 413.8,76.1 450.0,70.4 450.0,55.6 413.8,61.8 377.6,66.1 341.5,60.6 305.3,48.2 269.1,78.7 232.9,85.6 196.7,92.3 160.5,98.4 124.4,98.1 88.2,106.9 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,196.4 124.4,112.2 160.5,111.7 196.7,112.2 232.9,110.9 269.1,110.0 305.3,106.6 341.5,86.9 377.6,85.5 413.8,76.1 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,111.4 88.2,109.4 124.4,103.3 160.5,111.3 196.7,106.1 232.9,104.2 269.1,95.6 305.3,93.8 341.5,72.0 377.6,90.3 413.8,77.6 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,105.5 124.4,101.7 160.5,101.7 196.7,98.1 232.9,94.7 269.1,89.7 305.3,88.1 341.5,67.4 377.6,77.4 413.8,73.7 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,105.4 124.4,101.5 160.5,98.4 196.7,90.6 232.9,86.5 269.1,86.2 305.3,86.4 341.5,68.0 377.6,71.4 413.8,67.5 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,106.9 124.4,98.1 160.5,98.4 196.7,92.3 232.9,85.6 269.1,78.7 305.3,48.2 341.5,60.6 377.6,66.1 413.8,61.8 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.74 µs | 3.26 µs | 6.08 µs | 5.33 µs | 7.3 µs |
| D38 | 2.73 µs | 4.85 µs | 7.64 µs | 9.02 µs | 8.88 µs |
| D57 | 2.78 µs | 2.57 µs | 3.01 µs | 3.31 µs | 3 µs |
| D76 | 2.5 µs | 1.43 µs | 2.94 µs | 2.88 µs | 3.02 µs |
| D115 | 4.51 µs | 3.97 µs | 5.12 µs | 5.61 µs | 5.18 µs |
| D153 | 4.57 µs | 4.21 µs | 5.84 µs | 6.08 µs | 6.25 µs |
| D230 | 3.86 µs | 7.64 µs | 8.22 µs | 7.72 µs | 10.6 µs |
| D307 | 5.97 µs | 11.5 µs | 12.5 µs | 10.4 µs | 16.6 µs |
| D462 | 9.63 µs | 12.3 µs | 15.4 µs | 14.6 µs | 19.8 µs |
| D616 | 11.8 µs | 15.4 µs | 33.1 µs | 35.5 µs | 45.6 µs |
| D924 | 25.2 µs | 39.7 µs | 47.4 µs | 70.9 µs | 92.4 µs |
| D1232 | 32.6 µs | 66.5 µs | 96.1 µs | 99.6 µs | 154 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.9 88.2,180.9 124.4,180.3 160.5,183.5 196.7,166.4 232.9,166.0 269.1,170.9 305.3,158.2 341.5,144.4 377.6,138.5 413.8,116.5 450.0,109.2 450.0,64.1 413.8,78.9 377.6,99.4 341.5,123.6 305.3,128.7 269.1,141.6 232.9,156.9 196.7,162.4 160.5,178.0 124.4,178.2 88.2,146.8 52.0,152.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.9 88.2,180.9 124.4,180.3 160.5,183.5 196.7,166.4 232.9,166.0 269.1,170.9 305.3,158.2 341.5,144.4 377.6,138.5 413.8,116.5 450.0,109.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.8 88.2,164.3 124.4,182.6 160.5,199.6 196.7,170.1 232.9,168.4 269.1,151.1 305.3,139.3 341.5,137.3 377.6,130.9 413.8,103.4 450.0,88.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.8 88.2,151.1 124.4,178.1 160.5,178.7 196.7,162.7 232.9,158.9 269.1,149.0 305.3,136.8 341.5,130.8 377.6,108.7 413.8,98.3 450.0,77.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.3 124.4,175.3 160.5,179.4 196.7,160.1 232.9,157.7 269.1,150.8 305.3,142.3 341.5,132.3 377.6,106.7 413.8,86.6 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.5 88.2,146.8 124.4,178.2 160.5,178.0 196.7,162.4 232.9,156.9 269.1,141.6 305.3,128.7 341.5,123.6 377.6,99.4 413.8,78.9 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 2.25 µs | 4.19 µs | 3.51 µs | 4.67 µs |
| D38 | 6.55 ns | 3.33 µs | 4.83 µs | 5.36 µs | 4.59 µs |
| D57 | 62.2 ns | 4.17 µs | 4.91 µs | 5.39 µs | 5.25 µs |
| D76 | 74.2 ns | 2.27 µs | 5.08 µs | 5.22 µs | 5.56 µs |
| D115 | 137 ns | 6.88 µs | 9.05 µs | 10.3 µs | 9.98 µs |
| D153 | 178 ns | 7.02 µs | 10.3 µs | 11.1 µs | 12 µs |
| D230 | 164 ns | 13.6 µs | 15.2 µs | 16 µs | 22.9 µs |
| D307 | 214 ns | 20.5 µs | 21.5 µs | 21.3 µs | 34.5 µs |
| D462 | 686 ns | 70.9 µs | 148 µs | 201 µs | 284 µs |
| D616 | 594 ns | 119 µs | 353 µs | 356 µs | 572 µs |
| D924 | 991 ns | 424 µs | 383 µs | 791 µs | 1.74 ms |
| D1232 | 1.49 µs | 831 µs | 864 µs | 1.8 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.7 124.4,158.8 160.5,156.6 196.7,149.0 232.9,145.7 269.1,146.7 305.3,143.4 341.5,129.0 377.6,130.8 413.8,124.4 450.0,119.3 450.0,25.0 413.8,31.7 377.6,45.5 341.5,54.2 305.3,80.3 269.1,85.4 232.9,93.5 196.7,95.7 160.5,103.0 124.4,103.7 88.2,105.4 52.0,105.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.7 124.4,158.8 160.5,156.6 196.7,149.0 232.9,145.7 269.1,146.7 305.3,143.4 341.5,129.0 377.6,130.8 413.8,124.4 450.0,119.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.2 88.2,109.4 124.4,106.6 160.5,114.1 196.7,100.4 232.9,100.1 269.1,91.9 305.3,86.8 341.5,71.4 377.6,65.0 413.8,49.2 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,104.8 124.4,104.5 160.5,104.1 196.7,96.9 232.9,95.4 269.1,90.5 305.3,86.2 341.5,62.2 377.6,51.5 413.8,50.5 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,103.4 124.4,103.4 160.5,103.8 196.7,95.3 232.9,94.4 269.1,89.9 305.3,86.3 341.5,58.5 377.6,51.4 413.8,41.5 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.2 88.2,105.4 124.4,103.7 160.5,103.0 196.7,95.7 232.9,93.5 269.1,85.4 305.3,80.3 341.5,54.2 377.6,45.5 413.8,31.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.76 ns | 13.4 ns | 18.5 ns | 23.1 ns | 29.9 ns |
| D38 | 8.04 ns | 16.1 ns | 33.1 ns | 338 ns | 404 ns |
| D57 | 176 ns | 207 ns | 429 ns | 682 ns | 773 ns |
| D76 | 190 ns | 152 ns | 647 ns | 844 ns | 1.22 µs |
| D115 | 94.5 ns | 436 ns | 993 ns | 1.51 µs | 1.39 µs |
| D153 | 108 ns | 760 ns | 1.56 µs | 1.64 µs | 2.66 µs |
| D230 | 83.1 ns | 457 ns | 2.4 µs | 2.76 µs | 4.33 µs |
| D307 | 121 ns | 2.34 µs | 3.64 µs | 3.48 µs | 7.2 µs |
| D462 | 179 ns | 3.64 µs | 6.02 µs | 8.16 µs | 11.7 µs |
| D616 | 237 ns | 1.23 µs | 11 µs | 13.9 µs | 20.2 µs |
| D924 | 285 ns | 2.71 µs | 13.4 µs | 25.6 µs | 34.9 µs |
| D1232 | 319 ns | 16.5 µs | 28.1 µs | 31.8 µs | 61.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,173.8 124.4,120.2 160.5,118.9 196.7,131.0 232.9,128.7 269.1,133.2 305.3,126.6 341.5,119.9 377.6,115.0 413.8,111.8 450.0,109.9 450.0,18.3 413.8,28.3 377.6,37.8 341.5,47.3 305.3,55.7 269.1,64.5 232.9,73.0 196.7,84.3 160.5,86.6 124.4,94.5 88.2,105.8 52.0,151.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,173.8 124.4,120.2 160.5,118.9 196.7,131.0 232.9,128.7 269.1,133.2 305.3,126.6 341.5,119.9 377.6,115.0 413.8,111.8 450.0,109.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,161.8 124.4,117.3 160.5,122.7 196.7,104.4 232.9,94.8 269.1,103.6 305.3,75.2 341.5,67.6 377.6,86.4 413.8,72.7 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,149.2 124.4,104.7 160.5,97.6 196.7,90.1 232.9,82.2 269.1,74.8 305.3,67.5 341.5,58.8 377.6,48.3 413.8,44.9 450.0,32.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.5 88.2,108.9 124.4,96.7 160.5,92.9 196.7,82.8 232.9,81.4 269.1,72.4 305.3,68.3 341.5,53.5 377.6,44.2 413.8,33.7 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.0 88.2,105.8 124.4,94.5 160.5,86.6 196.7,84.3 232.9,73.0 269.1,64.5 305.3,55.7 341.5,47.3 377.6,37.8 413.8,28.3 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
