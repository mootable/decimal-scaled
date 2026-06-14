# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.07 µs | 4.98 µs | 2.46 µs | 7.7 µs | 7.71 µs |
| D38 | 5.07 µs | 5.81 µs | 8.7 µs | 10.9 µs | 10.9 µs |
| D57 | 346 ns | 595 ns | 659 ns | 1.13 µs | 1.06 µs |
| D76 | 535 ns | 765 ns | 1.12 µs | 1.36 µs | 1.94 µs |
| D115 | 305 ns | 2.32 µs | 2.42 µs | 3.52 µs | 5.27 µs |
| D153 | 225 ns | 2.64 µs | 3.62 µs | 5.77 µs | 6.81 µs |
| D230 | 389 ns | 4.19 µs | 8.33 µs | 10.3 µs | 14.2 µs |
| D307 | 419 ns | 5.59 µs | 11.2 µs | 16.6 µs | 21.2 µs |
| D462 | 630 ns | 10.4 µs | 26.2 µs | 35.9 µs | 51.7 µs |
| D616 | 684 ns | 15.8 µs | 40.8 µs | 63.3 µs | 85 µs |
| D924 | 720 ns | 31.2 µs | 82.3 µs | 133 µs | 180 µs |
| D1232 | 840 ns | 40.1 µs | 90.8 µs | 227 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,124.7 124.4,183.0 160.5,173.6 196.7,185.8 232.9,192.4 269.1,180.5 305.3,178.9 341.5,170.0 377.6,168.3 413.8,167.1 450.0,163.8 450.0,33.1 413.8,47.2 377.6,63.5 341.5,74.3 305.3,93.7 269.1,102.4 232.9,118.3 196.7,123.9 160.5,145.6 124.4,158.8 88.2,108.1 52.0,115.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,124.7 124.4,183.0 160.5,173.6 196.7,185.8 232.9,192.4 269.1,180.5 305.3,178.9 341.5,170.0 377.6,168.3 413.8,167.1 450.0,163.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.1 88.2,121.8 124.4,171.3 160.5,165.8 196.7,141.7 232.9,138.9 269.1,128.9 305.3,122.6 341.5,109.2 377.6,100.1 413.8,85.3 450.0,79.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,140.5 88.2,113.0 124.4,169.1 160.5,157.6 196.7,140.9 232.9,132.1 269.1,114.0 305.3,107.6 341.5,89.1 377.6,79.4 413.8,64.2 450.0,62.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,108.1 124.4,157.3 160.5,153.4 196.7,132.7 232.9,121.9 269.1,109.4 305.3,99.0 341.5,82.3 377.6,69.9 413.8,53.8 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.6 88.2,108.1 124.4,158.8 160.5,145.6 196.7,123.9 232.9,118.3 269.1,102.4 305.3,93.7 341.5,74.3 377.6,63.5 413.8,47.2 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 4.3 µs | 4.03 µs | 6.73 µs | 7.28 µs |
| D38 | 1.87 ns | 6.77 µs | 8.24 µs | 9.5 µs | 10.7 µs |
| D57 | 2.81 ns | 3.53 µs | 4.47 µs | 5.35 µs | 9.65 µs |
| D76 | 3.21 ns | 6.05 µs | 7.28 µs | 9.69 µs | 10.9 µs |
| D115 | 13.2 ns | 6.34 µs | 12.5 µs | 17.2 µs | 23.4 µs |
| D153 | 10.1 ns | 6.88 µs | 14.8 µs | 21.5 µs | 33.9 µs |
| D230 | 57.6 ns | 13.6 µs | 23 µs | 41.5 µs | 77 µs |
| D307 | 105 ns | 15.7 µs | 34.1 µs | 77.6 µs | 114 µs |
| D462 | 133 ns | 23.5 µs | 83.5 µs | 162 µs | 244 µs |
| D616 | 159 ns | 34.9 µs | 137 µs | 285 µs | 441 µs |
| D924 | 170 ns | 84.4 µs | 265 µs | 603 µs | 903 µs |
| D1232 | 399 ns | 109 µs | 228 µs | 901 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,202.2 124.4,197.2 160.5,195.5 196.7,178.0 232.9,181.3 269.1,159.7 305.3,152.2 341.5,149.3 377.6,147.1 413.8,146.3 450.0,135.7 450.0,25.6 413.8,39.8 377.6,48.7 341.5,56.1 305.3,65.5 269.1,70.4 232.9,80.6 196.7,85.2 160.5,94.6 124.4,96.2 88.2,94.8 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,202.2 124.4,197.2 160.5,195.5 196.7,178.0 232.9,181.3 269.1,159.7 305.3,152.2 341.5,149.3 377.6,147.1 413.8,146.3 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.2 88.2,100.5 124.4,108.6 160.5,102.0 196.7,101.4 232.9,100.3 269.1,91.9 305.3,90.1 341.5,85.1 377.6,80.2 413.8,69.2 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,98.1 124.4,105.7 160.5,99.7 196.7,92.9 232.9,90.9 269.1,85.4 305.3,80.5 341.5,69.4 377.6,63.2 413.8,55.1 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.6 88.2,96.4 124.4,103.5 160.5,96.1 196.7,89.0 232.9,86.2 269.1,78.1 305.3,70.3 341.5,61.1 377.6,54.2 413.8,44.8 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,94.8 124.4,96.2 160.5,94.6 196.7,85.2 232.9,80.6 269.1,70.4 305.3,65.5 341.5,56.1 377.6,48.7 413.8,39.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 38.4 ns | 40.2 ns | 40.2 ns | 39.5 ns |
| D38 | 15.9 ns | 42.8 ns | 74.4 ns | 75.4 ns | 108 ns |
| D57 | 16.5 ns | 40 ns | 74.6 ns | 687 ns | 643 ns |
| D76 | 17.3 ns | 73.2 ns | 602 ns | 621 ns | 1.04 µs |
| D115 | 15.9 ns | 72.5 ns | 703 ns | 1.21 µs | 1.26 µs |
| D153 | 13.4 ns | 696 ns | 1.05 µs | 1.4 µs | 2.09 µs |
| D230 | 28.2 ns | 643 ns | 1.38 µs | 2.39 µs | 3.29 µs |
| D307 | 42.7 ns | 965 ns | 2.14 µs | 3.31 µs | 5.59 µs |
| D462 | 63.5 ns | 1.43 µs | 3.6 µs | 6.5 µs | 9.04 µs |
| D616 | 76.6 ns | 2.5 µs | 6.16 µs | 11.4 µs | 15.4 µs |
| D924 | 99.7 ns | 3.74 µs | 11 µs | 24.9 µs | 28.5 µs |
| D1232 | 111 ns | 4.98 µs | 12 µs | 27.5 µs | 50.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,199.9 124.4,199.1 160.5,198.1 196.7,199.9 232.9,203.7 269.1,187.5 305.3,178.5 341.5,169.9 377.6,165.8 413.8,160.1 450.0,157.7 450.0,24.7 413.8,37.2 377.6,50.6 341.5,62.2 305.3,72.6 269.1,84.2 232.9,93.9 196.7,104.9 160.5,109.1 124.4,119.6 88.2,158.4 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,199.9 124.4,199.1 160.5,198.1 196.7,199.9 232.9,203.7 269.1,187.5 305.3,178.5 341.5,169.9 377.6,165.8 413.8,160.1 450.0,157.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.8 88.2,178.4 124.4,179.9 160.5,166.8 196.7,167.0 232.9,117.9 269.1,119.6 305.3,110.8 341.5,102.3 377.6,90.1 413.8,81.4 450.0,75.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,166.4 124.4,166.4 160.5,121.0 196.7,117.7 232.9,108.9 269.1,103.0 305.3,93.5 341.5,82.2 377.6,70.5 413.8,57.8 450.0,56.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,166.1 124.4,118.2 160.5,120.3 196.7,105.9 232.9,102.7 269.1,91.1 305.3,84.0 341.5,69.4 377.6,57.2 413.8,40.2 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,158.4 124.4,119.6 160.5,109.1 196.7,104.9 232.9,93.9 269.1,84.2 305.3,72.6 341.5,62.2 377.6,50.6 413.8,37.2 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 278 ns | 350 ns | 369 ns | 377 ns |
| D38 | 2.18 ns | 390 ns | 402 ns | 401 ns | 406 ns |
| D57 | 268 ns | 438 ns | 450 ns | 445 ns | 624 ns |
| D76 | 276 ns | 484 ns | 473 ns | 628 ns | 554 ns |
| D115 | 220 ns | 444 ns | 573 ns | 919 ns | 1.08 µs |
| D153 | 161 ns | 463 ns | 589 ns | 934 ns | 1.22 µs |
| D230 | 552 ns | 740 ns | 1.11 µs | 1.2 µs | 1.64 µs |
| D307 | 739 ns | 763 ns | 1.03 µs | 1.24 µs | 9.63 µs |
| D462 | 1.12 µs | 3.04 µs | 3.66 µs | 4.08 µs | 5.15 µs |
| D616 | 1.35 µs | 1.33 µs | 1.79 µs | 2.84 µs | 3.6 µs |
| D924 | 1.83 µs | 2.04 µs | 2.69 µs | 3.55 µs | 4.28 µs |
| D1232 | 3.06 µs | 2.42 µs | 2.19 µs | 4.61 µs | 6.41 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.1 88.2,193.1 124.4,88.6 160.5,88.0 196.7,92.9 232.9,99.6 269.1,72.9 305.3,66.6 341.5,57.5 377.6,53.5 413.8,46.9 450.0,35.7 450.0,19.7 413.8,28.4 377.6,32.2 341.5,24.4 305.3,10.8 269.1,49.2 232.9,55.7 196.7,58.3 160.5,72.8 124.4,70.2 88.2,79.6 52.0,81.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.1 88.2,193.1 124.4,88.6 160.5,88.0 196.7,92.9 232.9,99.6 269.1,72.9 305.3,66.6 341.5,57.5 377.6,53.5 413.8,46.9 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,87.8 88.2,80.5 124.4,77.9 160.5,75.8 196.7,77.6 232.9,76.7 269.1,66.5 305.3,65.9 341.5,35.8 377.6,53.8 413.8,44.6 450.0,40.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.8 88.2,79.8 124.4,77.3 160.5,76.2 196.7,72.1 232.9,71.5 269.1,57.6 305.3,59.3 341.5,31.8 377.6,47.3 413.8,38.5 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.6 88.2,79.8 124.4,77.6 160.5,70.1 196.7,61.8 232.9,61.5 269.1,56.0 305.3,55.4 341.5,29.5 377.6,37.3 413.8,32.5 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.2 88.2,79.6 124.4,70.2 160.5,72.8 196.7,58.3 232.9,55.7 269.1,49.2 305.3,10.8 341.5,24.4 377.6,32.2 413.8,28.4 450.0,19.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.7 µs | 8.56 µs | 7.41 µs | 12.7 µs | 13.9 µs |
| D38 | 7.71 µs | 12.4 µs | 15.6 µs | 18.3 µs | 21.2 µs |
| D57 | 4.47 µs | 4.09 µs | 4.88 µs | 4.38 µs | 5.21 µs |
| D76 | 4.57 µs | 4.87 µs | 4.92 µs | 5.1 µs | 4.76 µs |
| D115 | 6.91 µs | 8.28 µs | 8.55 µs | 9.3 µs | 10.5 µs |
| D153 | 4.52 µs | 8.27 µs | 8.89 µs | 9.71 µs | 10.5 µs |
| D230 | 10.9 µs | 12.7 µs | 14.5 µs | 14.7 µs | 16.7 µs |
| D307 | 16.2 µs | 19.2 µs | 21.3 µs | 23.7 µs | 26.1 µs |
| D462 | 15.8 µs | 21.7 µs | 25.2 µs | 29.6 µs | 33.6 µs |
| D616 | 27.7 µs | 39.6 µs | 55.7 µs | 66.8 µs | 78.6 µs |
| D924 | 40.3 µs | 73 µs | 96.2 µs | 133 µs | 149 µs |
| D1232 | 55.4 µs | 88.9 µs | 92.9 µs | 201 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.9 88.2,150.8 124.4,166.7 160.5,166.0 196.7,154.0 232.9,166.4 269.1,140.8 305.3,129.3 341.5,130.1 377.6,113.9 413.8,103.0 450.0,93.8 450.0,47.9 413.8,65.1 377.6,83.6 341.5,108.3 305.3,115.6 269.1,128.5 232.9,142.0 196.7,141.8 160.5,164.9 124.4,162.2 88.2,121.6 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.9 88.2,150.8 124.4,166.7 160.5,166.0 196.7,154.0 232.9,166.4 269.1,140.8 305.3,129.3 341.5,130.1 377.6,113.9 413.8,103.0 450.0,93.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,147.8 88.2,137.1 124.4,169.2 160.5,164.2 196.7,148.8 232.9,148.8 269.1,136.4 305.3,124.4 341.5,121.0 377.6,103.5 413.8,85.8 450.0,80.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.0 88.2,130.5 124.4,164.1 160.5,163.9 196.7,147.9 232.9,146.8 269.1,132.5 305.3,121.4 341.5,116.6 377.6,93.6 413.8,77.8 450.0,78.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.4 88.2,125.8 124.4,167.2 160.5,162.9 196.7,145.4 232.9,144.2 269.1,132.2 305.3,118.4 341.5,112.0 377.6,88.4 413.8,68.3 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,121.6 124.4,162.2 160.5,164.9 196.7,141.8 232.9,142.0 269.1,128.5 305.3,115.6 341.5,108.3 377.6,83.6 413.8,65.1 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.72 ns | 4.72 µs | 4.64 µs | 7.37 µs | 7.94 µs |
| D38 | 6.24 ns | 7.45 µs | 8.96 µs | 10.2 µs | 11.5 µs |
| D57 | 64.8 ns | 3.96 µs | 4.63 µs | 4.49 µs | 5.48 µs |
| D76 | 78 ns | 4.35 µs | 4.88 µs | 5.45 µs | 5.37 µs |
| D115 | 125 ns | 8.1 µs | 8.73 µs | 10 µs | 11.6 µs |
| D153 | 102 ns | 8.4 µs | 9.52 µs | 10.7 µs | 11.9 µs |
| D230 | 342 ns | 13.4 µs | 16.1 µs | 17.8 µs | 20.7 µs |
| D307 | 437 ns | 20.1 µs | 21.1 µs | 28 µs | 32 µs |
| D462 | 633 ns | 78.1 µs | 147 µs | 220 µs | 279 µs |
| D616 | 741 ns | 177 µs | 350 µs | 379 µs | 570 µs |
| D924 | 914 ns | 451 µs | 458 µs | 849 µs | 1.61 ms |
| D1232 | 1.46 µs | 645 µs | 446 µs | 2.11 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,187.3 124.4,158.2 160.5,155.9 196.7,150.1 232.9,152.6 269.1,137.6 305.3,134.6 341.5,130.0 377.6,128.0 413.8,125.4 450.0,119.6 450.0,25.0 413.8,32.6 377.6,45.5 341.5,54.4 305.3,81.3 269.1,86.7 232.9,93.6 196.7,93.9 160.5,103.4 124.4,103.2 88.2,94.0 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,187.3 124.4,158.2 160.5,155.9 196.7,150.1 232.9,152.6 269.1,137.6 305.3,134.6 341.5,130.0 377.6,128.0 413.8,125.4 450.0,119.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.0 88.2,99.4 124.4,107.2 160.5,106.0 196.7,98.3 232.9,97.9 269.1,92.1 305.3,87.1 341.5,70.2 377.6,60.0 413.8,48.5 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,97.1 124.4,105.3 160.5,104.6 196.7,97.4 232.9,96.3 269.1,89.8 305.3,86.4 341.5,62.4 377.6,51.6 413.8,48.3 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.5 88.2,95.4 124.4,105.6 160.5,103.2 196.7,95.7 232.9,94.9 269.1,88.5 305.3,82.9 341.5,57.4 377.6,50.6 413.8,40.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,94.0 124.4,103.2 160.5,103.4 196.7,93.9 232.9,93.6 269.1,86.7 305.3,81.3 341.5,54.4 377.6,45.5 413.8,32.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.37 ns | 14 ns | 20.6 ns | 29.9 ns | 28.9 ns |
| D38 | 7.51 ns | 36.5 ns | 34.9 ns | 1.64 µs | 3.19 µs |
| D57 | 176 ns | 206 ns | 432 ns | 766 ns | 670 ns |
| D76 | 211 ns | 278 ns | 644 ns | 721 ns | 1.21 µs |
| D115 | 118 ns | 661 ns | 1.01 µs | 1.53 µs | 1.64 µs |
| D153 | 67.1 ns | 1.11 µs | 1.59 µs | 1.92 µs | 2.73 µs |
| D230 | 158 ns | 1.68 µs | 2.38 µs | 3.45 µs | 4.32 µs |
| D307 | 156 ns | 2.32 µs | 3.64 µs | 4.62 µs | 6.95 µs |
| D462 | 178 ns | 3.73 µs | 6.18 µs | 9.9 µs | 11.7 µs |
| D616 | 236 ns | 5.97 µs | 11.1 µs | 14.3 µs | 20.7 µs |
| D924 | 216 ns | 11.4 µs | 17.1 µs | 26.8 µs | 34.7 µs |
| D1232 | 301 ns | 13.2 µs | 15.3 µs | 40 µs | 62.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.8 88.2,175.0 124.4,120.2 160.5,117.1 196.7,127.1 232.9,136.9 269.1,122.0 305.3,122.2 341.5,120.0 377.6,115.1 413.8,116.7 450.0,110.9 450.0,18.2 413.8,28.4 377.6,37.3 341.5,47.3 305.3,56.3 269.1,64.6 232.9,72.6 196.7,81.4 160.5,86.7 124.4,97.0 88.2,69.8 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.8 88.2,175.0 124.4,120.2 160.5,117.1 196.7,127.1 232.9,136.9 269.1,122.0 305.3,122.2 341.5,120.0 377.6,115.1 413.8,116.7 450.0,110.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.2 88.2,147.5 124.4,117.4 160.5,112.3 196.7,97.2 232.9,88.2 269.1,81.0 305.3,75.4 341.5,67.1 377.6,59.0 413.8,47.7 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.4 88.2,148.3 124.4,104.6 160.5,97.6 196.7,89.8 232.9,81.9 269.1,75.0 305.3,67.6 341.5,58.4 377.6,48.2 413.8,40.7 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.0 88.2,81.4 124.4,94.6 160.5,95.7 196.7,82.6 232.9,78.7 269.1,68.5 305.3,63.4 341.5,50.2 377.6,43.8 413.8,32.9 450.0,25.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,69.8 124.4,97.0 160.5,86.7 196.7,81.4 232.9,72.6 269.1,64.6 305.3,56.3 341.5,47.3 377.6,37.3 413.8,28.4 450.0,18.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
