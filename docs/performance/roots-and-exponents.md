# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.8 µs | 4.5 µs | 7.7 µs | 7.71 µs |
| D38 | 5.73 µs | 5.14 µs | 8.67 µs | 10.9 µs | 10.9 µs |
| D57 | 352 ns | 533 ns | 717 ns | 1.14 µs | 1.06 µs |
| D76 | 534 ns | 761 ns | 1.19 µs | 1.07 µs | 1.94 µs |
| D115 | 506 ns | 2.09 µs | 2.42 µs | 2.87 µs | 3.41 µs |
| D153 | 355 ns | 2.65 µs | 3.65 µs | 5.85 µs | 6.86 µs |
| D230 | 421 ns | 4.08 µs | 8.48 µs | 8.6 µs | 14.1 µs |
| D307 | 565 ns | 6.13 µs | 11.4 µs | 16.7 µs | 22.7 µs |
| D462 | 616 ns | 8.49 µs | 24.2 µs | 35.7 µs | 47.3 µs |
| D616 | 532 ns | 16.2 µs | 33.4 µs | 57.9 µs | 77.5 µs |
| D924 | 579 ns | 31.2 µs | 82.5 µs | 133 µs | 181 µs |
| D1232 | 832 ns | 48.8 µs | 150 µs | 204 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,122.1 124.4,182.6 160.5,173.6 196.7,174.8 232.9,182.5 269.1,178.8 305.3,172.4 341.5,170.5 377.6,173.7 413.8,171.9 450.0,164.0 450.0,33.1 413.8,47.2 377.6,65.5 341.5,76.3 305.3,92.2 269.1,102.5 232.9,118.2 196.7,133.4 160.5,145.6 124.4,158.8 88.2,108.2 52.0,115.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,122.1 124.4,182.6 160.5,173.6 196.7,174.8 232.9,182.5 269.1,178.8 305.3,172.4 341.5,170.5 377.6,173.7 413.8,171.9 450.0,164.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.8 88.2,124.4 124.4,173.7 160.5,165.9 196.7,144.0 232.9,138.8 269.1,129.4 305.3,120.6 341.5,113.6 377.6,99.5 413.8,85.3 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,127.3 88.2,113.1 124.4,167.2 160.5,156.3 196.7,140.8 232.9,131.9 269.1,113.6 305.3,107.2 341.5,90.8 377.6,83.8 413.8,64.2 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,108.1 124.4,157.2 160.5,158.6 196.7,137.1 232.9,121.6 269.1,113.3 305.3,98.8 341.5,82.4 377.6,71.9 413.8,53.8 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,108.2 124.4,158.8 160.5,145.6 196.7,133.4 232.9,118.2 269.1,102.5 305.3,92.2 341.5,76.3 377.6,65.5 413.8,47.2 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 4.79 µs | 5.27 µs | 6.73 µs | 7.27 µs |
| D38 | 2.11 ns | 6.25 µs | 8.23 µs | 9.5 µs | 10.7 µs |
| D57 | 2.81 ns | 3.95 µs | 4.09 µs | 5.5 µs | 9.76 µs |
| D76 | 3.21 ns | 6 µs | 6.71 µs | 7.46 µs | 10.9 µs |
| D115 | 17.5 ns | 6.91 µs | 13.5 µs | 15.9 µs | 14.9 µs |
| D153 | 21.7 ns | 7.35 µs | 15.6 µs | 23 µs | 34 µs |
| D230 | 57.5 ns | 12.6 µs | 23 µs | 39 µs | 77.1 µs |
| D307 | 84.3 ns | 14.8 µs | 34 µs | 77.6 µs | 123 µs |
| D462 | 136 ns | 20.3 µs | 79.6 µs | 162 µs | 227 µs |
| D616 | 174 ns | 38.1 µs | 121 µs | 264 µs | 408 µs |
| D924 | 226 ns | 85.9 µs | 263 µs | 602 µs | 906 µs |
| D1232 | 415 ns | 138 µs | 439 µs | 888 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,200.7 124.4,197.2 160.5,195.5 196.7,174.5 232.9,171.8 269.1,159.7 305.3,155.0 341.5,149.1 377.6,146.0 413.8,142.8 450.0,135.2 450.0,25.6 413.8,39.8 377.6,49.7 341.5,57.0 305.3,64.6 269.1,70.4 232.9,80.5 196.7,90.8 160.5,94.6 124.4,96.0 88.2,94.8 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,200.7 124.4,197.2 160.5,195.5 196.7,174.5 232.9,171.8 269.1,159.7 305.3,155.0 341.5,149.1 377.6,146.0 413.8,142.8 450.0,135.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,101.6 124.4,107.2 160.5,102.0 196.7,100.3 232.9,99.5 269.1,92.9 305.3,90.9 341.5,86.9 377.6,79.1 413.8,69.0 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.7 88.2,98.1 124.4,106.8 160.5,100.7 196.7,92.0 232.9,90.2 269.1,85.4 305.3,80.5 341.5,70.0 377.6,64.7 413.8,55.1 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.6 88.2,96.3 124.4,103.1 160.5,99.3 196.7,90.0 232.9,85.4 269.1,78.8 305.3,70.3 341.5,61.2 377.6,55.1 413.8,44.9 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,94.8 124.4,96.0 160.5,94.6 196.7,90.8 232.9,80.5 269.1,70.4 305.3,64.6 341.5,57.0 377.6,49.7 413.8,39.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 39.8 ns | 33.3 ns | 40.2 ns | 39.5 ns |
| D38 | 15.4 ns | 41.7 ns | 74.5 ns | 75.3 ns | 108 ns |
| D57 | 16.6 ns | 42.2 ns | 67.3 ns | 696 ns | 645 ns |
| D76 | 17.3 ns | 72.9 ns | 692 ns | 493 ns | 1.04 µs |
| D115 | 22.2 ns | 79.2 ns | 626 ns | 852 ns | 828 ns |
| D153 | 23.1 ns | 614 ns | 1 µs | 1.26 µs | 2.1 µs |
| D230 | 28.3 ns | 724 ns | 1.38 µs | 2.05 µs | 3.33 µs |
| D307 | 41.5 ns | 1.09 µs | 2.14 µs | 3.32 µs | 5.66 µs |
| D462 | 69.7 ns | 1.25 µs | 3.62 µs | 6.39 µs | 9.6 µs |
| D616 | 72.7 ns | 2.32 µs | 5.62 µs | 10.8 µs | 15.9 µs |
| D924 | 119 ns | 3.82 µs | 11 µs | 24.8 µs | 28.5 µs |
| D1232 | 110 ns | 6.24 µs | 20.7 µs | 25.8 µs | 50.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,200.6 124.4,199.1 160.5,198.1 196.7,192.7 232.9,191.8 269.1,187.4 305.3,179.1 341.5,167.8 377.6,166.9 413.8,156.2 450.0,158.0 450.0,24.7 413.8,37.2 377.6,49.9 341.5,60.9 305.3,72.4 269.1,83.9 232.9,93.9 196.7,114.1 160.5,109.1 124.4,119.5 88.2,158.4 52.0,180.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,200.6 124.4,199.1 160.5,198.1 196.7,192.7 232.9,191.8 269.1,187.4 305.3,179.1 341.5,167.8 377.6,166.9 413.8,156.2 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,179.0 124.4,178.7 160.5,166.9 196.7,165.1 232.9,120.6 269.1,117.0 305.3,108.2 341.5,105.2 377.6,91.7 413.8,80.9 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,166.4 124.4,168.6 160.5,118.0 196.7,120.2 232.9,109.9 269.1,103.0 305.3,93.5 341.5,82.0 377.6,72.5 413.8,57.8 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,166.2 124.4,117.9 160.5,125.4 196.7,113.5 232.9,104.9 269.1,94.4 305.3,84.0 341.5,69.7 377.6,58.4 413.8,40.2 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.2 88.2,158.4 124.4,119.5 160.5,109.1 196.7,114.1 232.9,93.9 269.1,83.9 305.3,72.4 341.5,60.9 377.6,49.9 413.8,37.2 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 301 ns | 301 ns | 369 ns | 377 ns |
| D38 | 2.11 ns | 359 ns | 402 ns | 401 ns | 406 ns |
| D57 | 269 ns | 470 ns | 440 ns | 440 ns | 630 ns |
| D76 | 278 ns | 479 ns | 441 ns | 488 ns | 557 ns |
| D115 | 275 ns | 486 ns | 652 ns | 897 ns | 675 ns |
| D153 | 328 ns | 513 ns | 687 ns | 1.09 µs | 1.23 µs |
| D230 | 555 ns | 606 ns | 1.09 µs | 1.07 µs | 1.59 µs |
| D307 | 639 ns | 647 ns | 1.02 µs | 1.22 µs | 9.95 µs |
| D462 | 1.14 µs | 2.57 µs | 3.26 µs | 4.13 µs | 4.8 µs |
| D616 | 1.5 µs | 1.5 µs | 1.42 µs | 2.62 µs | 3.37 µs |
| D924 | 1.95 µs | 1.95 µs | 2.68 µs | 3.56 µs | 4.26 µs |
| D1232 | 3 µs | 2.99 µs | 4.07 µs | 3.94 µs | 6.48 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.7 88.2,193.8 124.4,88.5 160.5,87.8 196.7,88.1 232.9,84.2 269.1,72.8 305.3,69.7 341.5,57.2 377.6,51.2 413.8,45.5 450.0,36.2 450.0,19.4 413.8,28.5 377.6,33.6 341.5,26.0 305.3,10.1 269.1,49.9 232.9,55.6 196.7,68.5 160.5,72.7 124.4,70.0 88.2,79.6 52.0,81.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.7 88.2,193.8 124.4,88.5 160.5,87.8 196.7,88.1 232.9,84.2 269.1,72.8 305.3,69.7 341.5,57.2 377.6,51.2 413.8,45.5 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.1 88.2,82.3 124.4,76.4 160.5,76.0 196.7,75.7 232.9,74.5 269.1,70.9 305.3,69.5 341.5,39.5 377.6,51.2 413.8,45.4 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,86.1 88.2,79.8 124.4,77.8 160.5,77.8 196.7,69.3 232.9,68.1 269.1,58.1 305.3,59.5 341.5,34.3 377.6,52.4 413.8,38.6 450.0,29.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.6 88.2,79.9 124.4,77.8 160.5,75.6 196.7,62.4 232.9,58.2 269.1,58.6 305.3,55.7 341.5,29.2 377.6,39.1 413.8,32.4 450.0,30.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.2 88.2,79.6 124.4,70.0 160.5,72.7 196.7,68.5 232.9,55.6 269.1,49.9 305.3,10.1 341.5,26.0 377.6,33.6 413.8,28.5 450.0,19.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.16 µs | 9.23 µs | 9.63 µs | 12.7 µs | 13.9 µs |
| D38 | 8.14 µs | 11.6 µs | 15.5 µs | 18.3 µs | 21.1 µs |
| D57 | 4.51 µs | 4.65 µs | 4.47 µs | 4.4 µs | 5.22 µs |
| D76 | 4.48 µs | 4.93 µs | 4.36 µs | 4.01 µs | 4.71 µs |
| D115 | 7.51 µs | 9.15 µs | 9.3 µs | 8.45 µs | 6.61 µs |
| D153 | 8.24 µs | 9.16 µs | 9.79 µs | 10.8 µs | 10.6 µs |
| D230 | 10.8 µs | 11.8 µs | 14.7 µs | 12.7 µs | 16.6 µs |
| D307 | 15.9 µs | 18.2 µs | 21.4 µs | 23.7 µs | 28.2 µs |
| D462 | 15.9 µs | 18.1 µs | 23.6 µs | 29.4 µs | 31.2 µs |
| D616 | 29 µs | 43 µs | 45.1 µs | 61 µs | 71.7 µs |
| D924 | 42.3 µs | 72.7 µs | 95.8 µs | 133 µs | 149 µs |
| D1232 | 56.3 µs | 114 µs | 166 µs | 183 µs | 271 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,149.3 124.4,166.4 160.5,166.6 196.7,151.6 232.9,148.9 269.1,141.1 305.3,129.9 341.5,129.9 377.6,112.5 413.8,101.6 450.0,93.3 450.0,47.8 413.8,65.1 377.6,86.3 341.5,110.3 305.3,113.3 269.1,128.7 232.9,141.7 196.7,155.3 160.5,165.2 124.4,162.2 88.2,121.7 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,149.3 124.4,166.4 160.5,166.6 196.7,151.6 232.9,148.9 269.1,141.1 305.3,129.9 341.5,129.9 377.6,112.5 413.8,101.6 450.0,93.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.7 88.2,139.0 124.4,165.5 160.5,163.8 196.7,145.9 232.9,145.9 269.1,138.6 305.3,126.0 341.5,126.2 377.6,101.1 413.8,85.9 450.0,73.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,144.4 88.2,130.6 124.4,166.6 160.5,167.4 196.7,145.4 232.9,144.0 269.1,132.2 305.3,121.3 341.5,118.5 377.6,99.7 413.8,77.9 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.4 88.2,125.8 124.4,167.1 160.5,169.8 196.7,148.2 232.9,141.2 269.1,136.3 305.3,118.3 341.5,112.1 377.6,91.0 413.8,68.3 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,121.7 124.4,162.2 160.5,165.2 196.7,155.3 232.9,141.7 269.1,128.7 305.3,113.3 341.5,110.3 377.6,86.3 413.8,65.1 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 5.28 µs | 5.79 µs | 7.38 µs | 7.94 µs |
| D38 | 6.49 ns | 6.88 µs | 8.95 µs | 10.2 µs | 11.5 µs |
| D57 | 63.9 ns | 4.45 µs | 4.37 µs | 4.6 µs | 5.49 µs |
| D76 | 78.5 ns | 4.35 µs | 4.48 µs | 4.25 µs | 5.34 µs |
| D115 | 137 ns | 8.73 µs | 9.5 µs | 8.95 µs | 7.25 µs |
| D153 | 191 ns | 9.03 µs | 10.2 µs | 11.9 µs | 11.9 µs |
| D230 | 342 ns | 12.4 µs | 16 µs | 16.1 µs | 20.6 µs |
| D307 | 367 ns | 19 µs | 20.9 µs | 28 µs | 34.5 µs |
| D462 | 631 ns | 68 µs | 136 µs | 222 µs | 261 µs |
| D616 | 806 ns | 193 µs | 326 µs | 353 µs | 531 µs |
| D924 | 998 ns | 450 µs | 454 µs | 848 µs | 1.62 ms |
| D1232 | 1.5 µs | 831 µs | 853 µs | 2.17 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.8 124.4,158.4 160.5,155.9 196.7,149.0 232.9,144.8 269.1,137.6 305.3,136.7 341.5,130.0 377.6,127.0 413.8,124.3 450.0,119.2 450.0,25.0 413.8,32.6 377.6,46.4 341.5,55.2 305.3,80.3 269.1,86.7 232.9,93.6 196.7,99.7 160.5,103.5 124.4,103.2 88.2,94.0 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.8 124.4,158.4 160.5,155.9 196.7,149.0 232.9,144.8 269.1,137.6 305.3,136.7 341.5,130.0 377.6,127.0 413.8,124.3 450.0,119.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,100.4 124.4,105.8 160.5,106.0 196.7,97.4 232.9,97.0 269.1,93.1 305.3,87.8 341.5,71.9 377.6,59.0 413.8,48.5 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.5 88.2,97.1 124.4,106.0 160.5,105.7 196.7,96.3 232.9,95.4 269.1,89.9 305.3,86.6 341.5,63.4 377.6,52.5 413.8,48.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.5 88.2,95.5 124.4,105.3 160.5,106.3 196.7,97.1 232.9,93.5 269.1,89.8 305.3,83.0 341.5,57.3 377.6,51.5 413.8,40.6 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,94.0 124.4,103.2 160.5,103.5 196.7,99.7 232.9,93.6 269.1,86.7 305.3,80.3 341.5,55.2 377.6,46.4 413.8,32.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.92 ns | 14.4 ns | 15.5 ns | 29.9 ns | 28.8 ns |
| D38 | 9 ns | 26.7 ns | 31.9 ns | 1.64 µs | 3.19 µs |
| D57 | 176 ns | 221 ns | 486 ns | 767 ns | 667 ns |
| D76 | 208 ns | 277 ns | 771 ns | 562 ns | 1.21 µs |
| D115 | 115 ns | 600 ns | 932 ns | 1.17 µs | 983 ns |
| D153 | 125 ns | 1.03 µs | 1.63 µs | 1.84 µs | 2.67 µs |
| D230 | 159 ns | 1.59 µs | 2.46 µs | 3.08 µs | 4.21 µs |
| D307 | 151 ns | 2.3 µs | 3.61 µs | 4.66 µs | 7.15 µs |
| D462 | 178 ns | 3.02 µs | 5.9 µs | 9.75 µs | 11.9 µs |
| D616 | 255 ns | 6.37 µs | 9.14 µs | 14.2 µs | 20.7 µs |
| D924 | 231 ns | 11.3 µs | 16.9 µs | 26.6 µs | 34.7 µs |
| D1232 | 300 ns | 16.4 µs | 28.2 µs | 35.5 µs | 62.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.4 88.2,171.8 124.4,120.2 160.5,117.3 196.7,127.5 232.9,126.1 269.1,122.0 305.3,122.8 341.5,120.0 377.6,113.8 413.8,115.5 450.0,110.9 450.0,18.1 413.8,28.4 377.6,37.4 341.5,46.9 305.3,55.8 269.1,65.0 232.9,72.9 196.7,90.3 160.5,86.7 124.4,97.0 88.2,69.8 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.4 88.2,171.8 124.4,120.2 160.5,117.3 196.7,127.5 232.9,126.1 269.1,122.0 305.3,122.8 341.5,120.0 377.6,113.8 413.8,115.5 450.0,110.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.7 88.2,152.9 124.4,116.2 160.5,112.3 196.7,98.9 232.9,89.6 269.1,82.0 305.3,75.5 341.5,70.8 377.6,57.8 413.8,47.8 450.0,41.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.4 88.2,149.8 124.4,102.5 160.5,94.5 196.7,91.2 232.9,81.5 269.1,74.4 305.3,67.7 341.5,59.2 377.6,51.6 413.8,40.9 450.0,32.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.9 88.2,81.4 124.4,94.6 160.5,100.0 196.7,87.2 232.9,79.4 269.1,70.4 305.3,63.3 341.5,50.4 377.6,43.9 413.8,33.0 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,69.8 124.4,97.0 160.5,86.7 196.7,90.3 232.9,72.9 269.1,65.0 305.3,55.8 341.5,46.9 377.6,37.4 413.8,28.4 450.0,18.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
