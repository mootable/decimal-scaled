# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.08 µs | 5.79 µs | 5.15 µs | 7.7 µs | 8.7 µs |
| D38 | 5.73 µs | 5.78 µs | 8.67 µs | 9.67 µs | 9.65 µs |
| D57 | 269 ns | 595 ns | 717 ns | 1.04 µs | 1.06 µs |
| D76 | 463 ns | 790 ns | 1.19 µs | 1.35 µs | 1.91 µs |
| D115 | 334 ns | 2.09 µs | 2.42 µs | 3.63 µs | 5.28 µs |
| D153 | 287 ns | 2.81 µs | 3.5 µs | 3.27 µs | 6.97 µs |
| D230 | 548 ns | 4.11 µs | 8.43 µs | 10.5 µs | 14.2 µs |
| D307 | 563 ns | 5.55 µs | 9.55 µs | 16.7 µs | 21.1 µs |
| D462 | 484 ns | 13.8 µs | 24.2 µs | 28.8 µs | 47.3 µs |
| D616 | 527 ns | 15.7 µs | 40.8 µs | 57.9 µs | 77.5 µs |
| D924 | 702 ns | 32.5 µs | 82.3 µs | 133 µs | 180 µs |
| D1232 | 827 ns | 27.7 µs | 151 µs | 226 µs | 268 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,122.1 124.4,188.5 160.5,176.7 196.7,183.8 232.9,187.1 269.1,173.1 305.3,172.5 341.5,175.8 377.6,173.9 413.8,167.7 450.0,164.1 450.0,38.6 413.8,47.2 377.6,65.5 341.5,76.2 305.3,93.8 269.1,102.4 232.9,117.8 196.7,123.9 160.5,145.9 124.4,158.8 88.2,110.8 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,122.1 124.4,188.5 160.5,176.7 196.7,183.8 232.9,187.1 269.1,173.1 305.3,172.5 341.5,175.8 377.6,173.9 413.8,167.7 450.0,164.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,121.9 124.4,171.3 160.5,165.1 196.7,144.0 232.9,137.5 269.1,129.3 305.3,122.8 341.5,103.0 377.6,100.2 413.8,84.4 450.0,87.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,113.1 124.4,167.2 160.5,156.2 196.7,140.8 232.9,132.8 269.1,113.7 305.3,111.0 341.5,90.8 377.6,79.5 413.8,64.2 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,110.7 124.4,159.2 160.5,153.4 196.7,132.0 232.9,134.3 269.1,109.0 305.3,98.9 341.5,87.0 377.6,71.9 413.8,53.8 450.0,42.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,110.8 124.4,158.8 160.5,145.9 196.7,123.9 232.9,117.8 269.1,102.4 305.3,93.8 341.5,76.2 377.6,65.5 413.8,47.2 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 4.79 µs | 6.25 µs | 6.73 µs | 7.88 µs |
| D38 | 2.11 ns | 6.77 µs | 8.23 µs | 8.73 µs | 9.86 µs |
| D57 | 2.18 ns | 3.55 µs | 4.09 µs | 5.93 µs | 9.66 µs |
| D76 | 3.12 ns | 5.51 µs | 6.75 µs | 9.78 µs | 11.9 µs |
| D115 | 17 ns | 6.85 µs | 12.5 µs | 17.3 µs | 23.1 µs |
| D153 | 20.4 ns | 6.86 µs | 15.9 µs | 12.4 µs | 37.3 µs |
| D230 | 51.4 ns | 13.8 µs | 23 µs | 41.2 µs | 76.9 µs |
| D307 | 84.2 ns | 15.8 µs | 31.9 µs | 77.4 µs | 115 µs |
| D462 | 166 ns | 23.3 µs | 79.1 µs | 128 µs | 227 µs |
| D616 | 176 ns | 38.4 µs | 139 µs | 265 µs | 409 µs |
| D924 | 173 ns | 79.8 µs | 263 µs | 601 µs | 905 µs |
| D1232 | 408 ns | 70.6 µs | 442 µs | 903 µs | 2.21 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,200.7 124.4,200.3 160.5,195.9 196.7,174.9 232.9,172.6 269.1,161.1 305.3,155.0 341.5,146.6 377.6,145.8 413.8,146.1 450.0,135.4 450.0,28.7 413.8,39.8 377.6,49.7 341.5,56.9 305.3,65.4 269.1,70.4 232.9,79.4 196.7,85.3 160.5,93.6 124.4,96.1 88.2,95.9 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,200.7 124.4,200.3 160.5,195.9 196.7,174.9 232.9,172.6 269.1,161.1 305.3,155.0 341.5,146.6 377.6,145.8 413.8,146.1 450.0,135.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,100.5 124.4,108.6 160.5,103.1 196.7,100.4 232.9,100.4 269.1,91.7 305.3,90.0 341.5,85.2 377.6,79.0 413.8,69.9 450.0,71.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,98.1 124.4,106.8 160.5,100.6 196.7,92.9 232.9,90.0 269.1,85.4 305.3,81.3 341.5,70.1 377.6,63.0 413.8,55.1 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.6 88.2,97.4 124.4,102.2 160.5,96.0 196.7,88.9 232.9,93.1 269.1,78.2 305.3,70.3 341.5,64.1 377.6,55.0 413.8,44.9 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,95.9 124.4,96.1 160.5,93.6 196.7,85.3 232.9,79.4 269.1,70.4 305.3,65.4 341.5,56.9 377.6,49.7 413.8,39.8 450.0,28.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 39.7 ns | 42.4 ns | 40.2 ns | 40.1 ns |
| D38 | 15.4 ns | 42.7 ns | 74.4 ns | 67.1 ns | 94.7 ns |
| D57 | 12.8 ns | 40 ns | 67.3 ns | 612 ns | 677 ns |
| D76 | 17.9 ns | 68.8 ns | 692 ns | 607 ns | 947 ns |
| D115 | 20.5 ns | 79.3 ns | 702 ns | 1.21 µs | 1.26 µs |
| D153 | 19.7 ns | 706 ns | 920 ns | 697 ns | 2.03 µs |
| D230 | 29.1 ns | 641 ns | 1.38 µs | 2.35 µs | 3.31 µs |
| D307 | 41.9 ns | 960 ns | 1.82 µs | 3.33 µs | 5.51 µs |
| D462 | 63.8 ns | 1.43 µs | 3.62 µs | 5.06 µs | 9.6 µs |
| D616 | 72.9 ns | 2.37 µs | 6.17 µs | 10.8 µs | 15.9 µs |
| D924 | 96.3 ns | 3.77 µs | 11 µs | 24.8 µs | 28.5 µs |
| D1232 | 110 ns | 3.64 µs | 20.9 µs | 27.5 µs | 39.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.7 88.2,200.6 124.4,204.6 160.5,197.3 196.7,194.4 232.9,195.3 269.1,186.8 305.3,178.9 341.5,169.7 377.6,166.9 413.8,160.8 450.0,158.0 450.0,30.1 413.8,37.3 377.6,49.9 341.5,60.9 305.3,72.9 269.1,84.0 232.9,94.6 196.7,105.0 160.5,111.2 124.4,118.5 88.2,161.2 52.0,179.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.7 88.2,200.6 124.4,204.6 160.5,197.3 196.7,194.4 232.9,195.3 269.1,186.8 305.3,178.9 341.5,169.7 377.6,166.9 413.8,160.8 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,178.5 124.4,179.9 160.5,168.1 196.7,165.0 232.9,117.6 269.1,119.7 305.3,110.9 341.5,102.2 377.6,91.3 413.8,81.2 450.0,82.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,166.4 124.4,168.6 160.5,118.0 196.7,117.7 232.9,111.8 269.1,103.1 305.3,97.0 341.5,82.0 377.6,70.5 413.8,57.8 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,168.7 124.4,120.6 160.5,120.9 196.7,105.9 232.9,117.8 269.1,91.5 305.3,83.9 341.5,74.8 377.6,58.4 413.8,40.3 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,161.2 124.4,118.5 160.5,111.2 196.7,105.0 232.9,94.6 269.1,84.0 305.3,72.9 341.5,60.9 377.6,49.9 413.8,37.3 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 301 ns | 374 ns | 369 ns | 400 ns |
| D38 | 2.11 ns | 392 ns | 402 ns | 368 ns | 372 ns |
| D57 | 208 ns | 437 ns | 438 ns | 481 ns | 627 ns |
| D76 | 270 ns | 448 ns | 437 ns | 632 ns | 650 ns |
| D115 | 285 ns | 487 ns | 562 ns | 918 ns | 1.06 µs |
| D153 | 289 ns | 462 ns | 694 ns | 603 ns | 1.42 µs |
| D230 | 474 ns | 717 ns | 1.11 µs | 1.23 µs | 1.61 µs |
| D307 | 637 ns | 756 ns | 886 ns | 1.25 µs | 9.67 µs |
| D462 | 1.3 µs | 3.08 µs | 3.27 µs | 3.36 µs | 4.76 µs |
| D616 | 1.51 µs | 1.56 µs | 1.78 µs | 2.62 µs | 3.43 µs |
| D924 | 1.79 µs | 1.8 µs | 2.68 µs | 3.58 µs | 4.28 µs |
| D1232 | 3.01 µs | 1.54 µs | 4.02 µs | 4.58 µs | 5.36 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.0 88.2,193.8 124.4,94.1 160.5,88.5 196.7,87.2 232.9,87.0 269.1,76.2 305.3,69.8 341.5,54.4 377.6,51.0 413.8,47.4 450.0,36.1 450.0,23.5 413.8,28.4 377.6,33.2 341.5,26.1 305.3,10.7 269.1,49.7 232.9,52.4 196.7,58.7 160.5,69.3 124.4,70.2 88.2,81.5 52.0,79.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.0 88.2,193.8 124.4,94.1 160.5,88.5 196.7,87.2 232.9,87.0 269.1,76.2 305.3,69.8 341.5,54.4 377.6,51.0 413.8,47.4 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.1 88.2,80.3 124.4,78.0 160.5,77.4 196.7,75.6 232.9,76.7 269.1,67.2 305.3,66.1 341.5,35.6 377.6,50.4 413.8,47.3 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.3 88.2,79.8 124.4,77.9 160.5,78.0 196.7,72.5 232.9,67.9 269.1,57.8 305.3,62.6 341.5,34.3 377.6,47.5 413.8,38.6 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.7 88.2,81.7 124.4,75.9 160.5,70.0 196.7,61.9 232.9,71.0 269.1,55.4 305.3,55.2 341.5,33.7 377.6,39.1 413.8,32.3 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.9 88.2,81.5 124.4,70.2 160.5,69.3 196.7,58.7 232.9,52.4 269.1,49.7 305.3,10.7 341.5,26.1 377.6,33.2 413.8,28.4 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.7 µs | 9.23 µs | 11.6 µs | 12.7 µs | 14.9 µs |
| D38 | 8.15 µs | 12.4 µs | 15.5 µs | 17.1 µs | 19.7 µs |
| D57 | 3.44 µs | 4.1 µs | 4.32 µs | 4.87 µs | 5.24 µs |
| D76 | 3.92 µs | 4.29 µs | 4.35 µs | 5.11 µs | 5.35 µs |
| D115 | 8.07 µs | 8.93 µs | 8.56 µs | 9.45 µs | 10.5 µs |
| D153 | 6.84 µs | 8.33 µs | 9.76 µs | 5.92 µs | 11.5 µs |
| D230 | 10.3 µs | 12.7 µs | 14.8 µs | 14.8 µs | 16.6 µs |
| D307 | 16 µs | 19.4 µs | 18.8 µs | 23.6 µs | 26.1 µs |
| D462 | 16.7 µs | 21.5 µs | 23.6 µs | 23.2 µs | 31.2 µs |
| D616 | 28.6 µs | 42.8 µs | 55.5 µs | 61 µs | 71.3 µs |
| D924 | 40.6 µs | 68 µs | 95.8 µs | 133 µs | 150 µs |
| D1232 | 55.5 µs | 61.5 µs | 165 µs | 200 µs | 215 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.9 88.2,149.3 124.4,174.2 160.5,170.4 196.7,149.5 232.9,154.3 269.1,142.3 305.3,129.8 341.5,128.5 377.6,112.9 413.8,102.7 450.0,93.7 450.0,54.6 413.8,65.0 377.6,86.5 341.5,110.4 305.3,115.6 269.1,128.7 232.9,139.2 196.7,141.8 160.5,161.4 124.4,162.0 88.2,123.6 52.0,131.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.9 88.2,149.3 124.4,174.2 160.5,170.4 196.7,149.5 232.9,154.3 269.1,142.3 305.3,129.8 341.5,128.5 377.6,112.9 413.8,102.7 450.0,93.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.6 88.2,137.1 124.4,169.2 160.5,167.8 196.7,146.6 232.9,148.6 269.1,136.3 305.3,124.1 341.5,121.1 377.6,101.3 413.8,87.8 450.0,90.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,130.6 124.4,167.6 160.5,167.4 196.7,147.8 232.9,144.0 269.1,131.9 305.3,125.1 341.5,118.5 377.6,93.7 413.8,77.9 450.0,62.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.4 88.2,127.8 124.4,164.1 160.5,162.8 196.7,145.0 232.9,158.5 269.1,131.9 305.3,118.5 341.5,119.0 377.6,91.0 413.8,68.4 450.0,56.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,123.6 124.4,162.0 160.5,161.4 196.7,141.8 232.9,139.2 269.1,128.7 305.3,115.6 341.5,110.4 377.6,86.5 413.8,65.0 450.0,54.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 5.28 µs | 6.88 µs | 7.38 µs | 8.59 µs |
| D38 | 6.48 ns | 7.45 µs | 8.94 µs | 9.38 µs | 10.6 µs |
| D57 | 63 ns | 3.93 µs | 4.3 µs | 4.93 µs | 5.5 µs |
| D76 | 69.4 ns | 3.85 µs | 4.54 µs | 5.47 µs | 5.84 µs |
| D115 | 149 ns | 8.58 µs | 8.75 µs | 10.1 µs | 11.6 µs |
| D153 | 158 ns | 8.42 µs | 10.2 µs | 6.39 µs | 13 µs |
| D230 | 287 ns | 13.2 µs | 16.1 µs | 17.7 µs | 20.7 µs |
| D307 | 371 ns | 20.1 µs | 18.9 µs | 28.1 µs | 31.7 µs |
| D462 | 687 ns | 76.7 µs | 135 µs | 174 µs | 262 µs |
| D616 | 809 ns | 192 µs | 351 µs | 354 µs | 525 µs |
| D924 | 920 ns | 422 µs | 454 µs | 845 µs | 1.61 ms |
| D1232 | 1.55 µs | 443 µs | 858 µs | 2.1 ms | 2.31 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.8 124.4,158.6 160.5,157.4 196.7,147.9 232.9,147.2 269.1,139.8 305.3,136.6 341.5,128.9 377.6,126.9 413.8,125.3 450.0,118.9 450.0,28.2 413.8,32.6 377.6,46.6 341.5,55.2 305.3,81.4 269.1,86.7 232.9,92.5 196.7,93.9 160.5,102.4 124.4,103.1 88.2,95.0 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.8 124.4,158.6 160.5,157.4 196.7,147.9 232.9,147.2 269.1,139.8 305.3,136.6 341.5,128.9 377.6,126.9 413.8,125.3 450.0,118.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,99.4 124.4,107.3 160.5,107.5 196.7,97.6 232.9,97.9 269.1,92.3 305.3,87.1 341.5,70.4 377.6,59.1 413.8,49.3 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.4 88.2,97.1 124.4,106.2 160.5,105.5 196.7,97.4 232.9,95.5 269.1,89.8 305.3,87.8 341.5,63.4 377.6,51.6 413.8,48.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.5 88.2,96.5 124.4,104.5 160.5,103.2 196.7,95.6 232.9,101.3 269.1,88.6 305.3,82.9 341.5,60.3 377.6,51.5 413.8,40.7 450.0,29.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,95.0 124.4,103.1 160.5,102.4 196.7,93.9 232.9,92.5 269.1,86.7 305.3,81.4 341.5,55.2 377.6,46.6 413.8,32.6 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.38 ns | 14.4 ns | 19.3 ns | 30 ns | 30.7 ns |
| D38 | 8.96 ns | 36.5 ns | 34.8 ns | 1.47 µs | 2.85 µs |
| D57 | 160 ns | 206 ns | 485 ns | 679 ns | 670 ns |
| D76 | 192 ns | 270 ns | 771 ns | 721 ns | 1.05 µs |
| D115 | 133 ns | 601 ns | 1.01 µs | 1.54 µs | 1.65 µs |
| D153 | 113 ns | 1.11 µs | 1.53 µs | 993 ns | 2.67 µs |
| D230 | 145 ns | 1.7 µs | 2.46 µs | 3.51 µs | 4.29 µs |
| D307 | 145 ns | 2.3 µs | 3.08 µs | 4.65 µs | 6.94 µs |
| D462 | 187 ns | 3.86 µs | 5.86 µs | 8 µs | 12 µs |
| D616 | 245 ns | 6.44 µs | 11.2 µs | 14.2 µs | 20.5 µs |
| D924 | 218 ns | 10.4 µs | 16.9 µs | 27.3 µs | 34.8 µs |
| D1232 | 289 ns | 9.1 µs | 28.3 µs | 39.3 µs | 49.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.8 88.2,171.9 124.4,121.9 160.5,118.7 196.7,125.0 232.9,127.8 269.1,123.6 305.3,123.6 341.5,119.1 377.6,114.4 413.8,116.5 450.0,111.5 450.0,22.3 413.8,28.3 377.6,37.5 341.5,46.8 305.3,56.3 269.1,64.7 232.9,72.9 196.7,81.3 160.5,89.2 124.4,97.0 88.2,71.8 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.8 88.2,171.9 124.4,121.9 160.5,118.7 196.7,125.0 232.9,127.8 269.1,123.6 305.3,123.6 341.5,119.1 377.6,114.4 413.8,116.5 450.0,111.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.6 88.2,147.5 124.4,117.4 160.5,112.7 196.7,98.8 232.9,88.1 269.1,80.8 305.3,75.6 341.5,66.6 377.6,57.6 413.8,49.3 450.0,51.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,148.3 124.4,102.6 160.5,94.5 196.7,89.8 232.9,82.6 269.1,74.4 305.3,70.4 341.5,59.3 377.6,48.1 413.8,40.8 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.9 88.2,83.3 124.4,96.7 160.5,95.7 196.7,82.5 232.9,90.1 269.1,68.2 305.3,63.3 341.5,53.9 377.6,43.9 413.8,32.6 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,71.8 124.4,97.0 160.5,89.2 196.7,81.3 232.9,72.9 269.1,64.7 305.3,56.3 341.5,46.8 377.6,37.5 413.8,28.3 450.0,22.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
