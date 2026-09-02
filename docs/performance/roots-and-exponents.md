# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 474 ns | 438 ns | 159 ns | 302 ns | 204 ns |
| D38 | 474 ns | 278 ns | 532 ns | 424 ns | 722 ns |
| D57 | 340 ns | 553 ns | 706 ns | 1.14 µs | 1.14 µs |
| D76 | 458 ns | 747 ns | 1.11 µs | 1.34 µs | 1.28 µs |
| D115 | 501 ns | 2.3 µs | 2.39 µs | 3.53 µs | 5.15 µs |
| D153 | 355 ns | 2.64 µs | 2.83 µs | 5.65 µs | 6.77 µs |
| D230 | 387 ns | 4.11 µs | 7.75 µs | 7.52 µs | 11.4 µs |
| D307 | 571 ns | 5.66 µs | 9.53 µs | 16.3 µs | 22.4 µs |
| D462 | 625 ns | 10.6 µs | 20.7 µs | 32.8 µs | 51.7 µs |
| D616 | 534 ns | 14.1 µs | 23.8 µs | 57.4 µs | 77 µs |
| D924 | 702 ns | 30 µs | 89.6 µs | 133 µs | 180 µs |
| D1232 | 847 ns | 37.6 µs | 151 µs | 227 µs | 313 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.2 88.2,176.2 124.4,183.4 160.5,176.9 196.7,175.0 232.9,182.5 269.1,180.6 305.3,172.2 341.5,170.2 377.6,173.6 413.8,167.7 450.0,163.6 450.0,35.2 413.8,47.2 377.6,65.7 341.5,74.3 305.3,92.5 269.1,107.1 232.9,118.5 196.7,124.4 160.5,154.6 124.4,157.1 88.2,167.1 52.0,194.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.2 88.2,176.2 124.4,183.4 160.5,176.9 196.7,175.0 232.9,182.5 269.1,180.6 305.3,172.2 341.5,170.2 377.6,173.6 413.8,167.7 450.0,163.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.9 88.2,187.8 124.4,172.9 160.5,166.3 196.7,141.9 232.9,138.9 269.1,129.3 305.3,122.4 341.5,108.8 377.6,102.6 413.8,86.2 450.0,81.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,199.9 88.2,173.7 124.4,167.6 160.5,157.8 196.7,141.1 232.9,137.4 269.1,115.5 305.3,111.0 341.5,94.2 377.6,91.2 413.8,62.4 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.0 88.2,178.6 124.4,157.1 160.5,153.7 196.7,132.6 232.9,122.4 269.1,116.2 305.3,99.4 341.5,84.2 377.6,72.1 413.8,53.8 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.5 88.2,167.1 124.4,157.1 160.5,154.6 196.7,124.4 232.9,118.5 269.1,107.1 305.3,92.5 341.5,74.3 377.6,65.7 413.8,47.2 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.83 µs | 3.56 µs | 3.8 µs | 2.22 µs |
| D38 | 1.56 ns | 3.56 µs | 3.83 µs | 3.47 µs | 5.21 µs |
| D57 | 2.81 ns | 4.2 µs | 4.44 µs | 5.82 µs | 9.31 µs |
| D76 | 3.12 ns | 6.4 µs | 7.65 µs | 9.22 µs | 8.32 µs |
| D115 | 11.9 ns | 6.59 µs | 13.1 µs | 19.1 µs | 22.1 µs |
| D153 | 18 ns | 7.27 µs | 13.5 µs | 23.7 µs | 38.3 µs |
| D230 | 45.4 ns | 14.1 µs | 22.2 µs | 31.9 µs | 65.5 µs |
| D307 | 68 ns | 15.8 µs | 31.7 µs | 79.5 µs | 123 µs |
| D462 | 110 ns | 24 µs | 70.3 µs | 155 µs | 246 µs |
| D616 | 141 ns | 34.7 µs | 74.1 µs | 266 µs | 409 µs |
| D924 | 181 ns | 86.1 µs | 288 µs | 603 µs | 915 µs |
| D1232 | 374 ns | 110 µs | 442 µs | 917 µs | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,204.5 124.4,197.2 160.5,195.9 196.7,179.3 232.9,174.2 269.1,162.7 305.3,157.6 341.5,151.7 377.6,148.6 413.8,145.5 450.0,136.5 450.0,25.0 413.8,39.7 377.6,49.7 341.5,56.0 305.3,64.5 269.1,72.4 232.9,79.1 196.7,85.9 160.5,98.0 124.4,96.6 88.2,103.8 52.0,114.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,204.5 124.4,197.2 160.5,195.9 196.7,179.3 232.9,174.2 269.1,162.7 305.3,157.6 341.5,151.7 377.6,148.6 413.8,145.5 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.8 88.2,108.5 124.4,106.5 160.5,101.3 196.7,100.9 232.9,99.7 269.1,91.4 305.3,90.0 341.5,84.9 377.6,80.3 413.8,69.0 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,107.6 124.4,105.8 160.5,99.0 196.7,92.3 232.9,92.0 269.1,85.8 305.3,81.4 341.5,71.5 377.6,70.9 413.8,54.0 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,108.8 124.4,102.4 160.5,96.7 196.7,87.7 232.9,85.0 269.1,81.3 305.3,70.0 341.5,61.7 377.6,55.0 413.8,44.9 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.4 88.2,103.8 124.4,96.6 160.5,98.0 196.7,85.9 232.9,79.1 269.1,72.4 305.3,64.5 341.5,56.0 377.6,49.7 413.8,39.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17 ns | 41.1 ns | 42.8 ns | 41.8 ns | 29 ns |
| D38 | 16.1 ns | 42.9 ns | 66.4 ns | 64.1 ns | 106 ns |
| D57 | 16.3 ns | 41.5 ns | 68 ns | 709 ns | 709 ns |
| D76 | 17.9 ns | 73.9 ns | 657 ns | 696 ns | 621 ns |
| D115 | 21.3 ns | 72.8 ns | 702 ns | 1.17 µs | 1.37 µs |
| D153 | 23.4 ns | 712 ns | 722 ns | 1.27 µs | 1.98 µs |
| D230 | 28.6 ns | 633 ns | 1.51 µs | 1.61 µs | 2.71 µs |
| D307 | 43.1 ns | 1.1 µs | 1.69 µs | 3.26 µs | 5.59 µs |
| D462 | 63.5 ns | 1.48 µs | 3.21 µs | 6.26 µs | 9.07 µs |
| D616 | 83.8 ns | 2.46 µs | 3.7 µs | 10.9 µs | 16.1 µs |
| D924 | 98.5 ns | 3.66 µs | 11.3 µs | 24.8 µs | 28 µs |
| D1232 | 123 ns | 4.91 µs | 20.8 µs | 27.3 µs | 49.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,199.7 124.4,199.4 160.5,197.4 196.7,193.6 232.9,191.5 269.1,187.2 305.3,178.3 341.5,169.9 377.6,163.8 413.8,160.3 450.0,155.5 450.0,25.1 413.8,37.6 377.6,49.6 341.5,62.1 305.3,72.6 269.1,88.3 232.9,95.2 196.7,103.1 160.5,120.3 124.4,117.5 88.2,158.6 52.0,186.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,199.7 124.4,199.4 160.5,197.4 196.7,193.6 232.9,191.5 269.1,187.2 305.3,178.3 341.5,169.9 377.6,163.8 413.8,160.3 450.0,155.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.3 88.2,178.4 124.4,179.1 160.5,166.6 196.7,166.9 232.9,117.4 269.1,119.9 305.3,107.9 341.5,101.5 377.6,90.5 413.8,81.8 450.0,75.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.9 124.4,168.4 160.5,119.1 196.7,117.7 232.9,117.1 269.1,101.1 305.3,98.6 341.5,84.7 377.6,81.6 413.8,57.3 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.9 88.2,169.7 124.4,117.5 160.5,117.9 196.7,106.5 232.9,104.9 269.1,99.7 305.3,84.3 341.5,70.2 377.6,58.1 413.8,40.3 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.9 88.2,158.6 124.4,117.5 160.5,120.3 196.7,103.1 232.9,95.2 269.1,88.3 305.3,72.6 341.5,62.1 377.6,49.6 413.8,37.6 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 305 ns | 413 ns | 415 ns | 266 ns |
| D38 | 2.49 ns | 405 ns | 389 ns | 357 ns | 426 ns |
| D57 | 280 ns | 512 ns | 482 ns | 488 ns | 631 ns |
| D76 | 278 ns | 506 ns | 512 ns | 608 ns | 418 ns |
| D115 | 282 ns | 492 ns | 621 ns | 1.08 µs | 970 ns |
| D153 | 362 ns | 535 ns | 594 ns | 1.11 µs | 1.42 µs |
| D230 | 567 ns | 729 ns | 1.02 µs | 1.1 µs | 1.57 µs |
| D307 | 648 ns | 687 ns | 1.09 µs | 1.35 µs | 11.4 µs |
| D462 | 1.16 µs | 3.13 µs | 3.06 µs | 4.05 µs | 5.69 µs |
| D616 | 1.58 µs | 1.4 µs | 1.13 µs | 2.92 µs | 3.69 µs |
| D924 | 1.98 µs | 2.22 µs | 3.15 µs | 4.01 µs | 4.93 µs |
| D1232 | 3.18 µs | 2.63 µs | 4.47 µs | 5.06 µs | 6.95 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,194.1 124.4,112.1 160.5,112.2 196.7,112.0 232.9,107.6 269.1,99.9 305.3,97.5 341.5,87.4 377.6,82.0 413.8,78.1 450.0,69.9 450.0,56.3 413.8,62.3 377.6,67.3 341.5,59.8 305.3,47.8 269.1,82.2 232.9,84.0 196.7,90.5 160.5,105.1 124.4,98.0 88.2,104.8 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,194.1 124.4,112.1 160.5,112.2 196.7,112.0 232.9,107.6 269.1,99.9 305.3,97.5 341.5,87.4 377.6,82.0 413.8,78.1 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.6 88.2,105.7 124.4,101.6 160.5,101.8 196.7,102.3 232.9,100.9 269.1,95.5 305.3,96.5 341.5,70.2 377.6,84.1 413.8,76.2 450.0,73.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.4 88.2,106.4 124.4,102.7 160.5,101.6 196.7,98.3 232.9,99.1 269.1,89.7 305.3,88.6 341.5,70.6 377.6,87.8 413.8,70.1 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,107.9 124.4,102.5 160.5,98.6 196.7,88.7 232.9,88.2 269.1,88.4 305.3,84.8 341.5,65.7 377.6,71.4 413.8,65.9 450.0,61.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,104.8 124.4,98.0 160.5,105.1 196.7,90.5 232.9,84.0 269.1,82.2 305.3,47.8 341.5,59.8 377.6,67.3 413.8,62.3 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.81 µs | 3.25 µs | 5.91 µs | 6.43 µs | 4.37 µs |
| D38 | 2.82 µs | 5.9 µs | 7.21 µs | 7.03 µs | 9.97 µs |
| D57 | 4.43 µs | 4.57 µs | 4.5 µs | 4.47 µs | 4.71 µs |
| D76 | 4.11 µs | 4.7 µs | 4.91 µs | 4.64 µs | 3.31 µs |
| D115 | 7.88 µs | 8.77 µs | 9.05 µs | 10.4 µs | 10.1 µs |
| D153 | 8.45 µs | 8.77 µs | 8.54 µs | 11 µs | 11.9 µs |
| D230 | 11.3 µs | 13.2 µs | 14.4 µs | 10.8 µs | 14.8 µs |
| D307 | 15.7 µs | 18.4 µs | 18.2 µs | 24 µs | 28.7 µs |
| D462 | 15.6 µs | 21.6 µs | 20.4 µs | 28 µs | 33.9 µs |
| D616 | 28.6 µs | 40 µs | 31.7 µs | 61.8 µs | 72.4 µs |
| D924 | 41.1 µs | 73.9 µs | 104 µs | 135 µs | 151 µs |
| D1232 | 57.7 µs | 91.2 µs | 169 µs | 202 µs | 249 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.1 88.2,180.0 124.4,166.9 160.5,169.1 196.7,150.2 232.9,148.2 269.1,139.8 305.3,130.3 341.5,130.4 377.6,112.9 413.8,102.4 450.0,92.6 450.0,50.3 413.8,64.8 377.6,86.0 341.5,108.0 305.3,112.8 269.1,132.0 232.9,138.3 196.7,143.1 160.5,175.4 124.4,165.1 88.2,143.4 52.0,167.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.1 88.2,180.0 124.4,166.9 160.5,169.1 196.7,150.2 232.9,148.2 269.1,139.8 305.3,130.3 341.5,130.4 377.6,112.9 413.8,102.4 450.0,92.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.9 88.2,158.6 124.4,166.0 160.5,165.2 196.7,147.1 232.9,147.1 269.1,135.3 305.3,125.7 341.5,121.0 377.6,103.2 413.8,85.4 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,152.8 124.4,166.4 160.5,163.9 196.7,146.2 232.9,147.9 269.1,132.8 305.3,126.0 341.5,122.7 377.6,109.9 413.8,75.4 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.1 88.2,153.5 124.4,166.7 160.5,165.6 196.7,142.1 232.9,140.7 269.1,141.0 305.3,117.9 341.5,113.6 377.6,90.6 413.8,67.9 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.3 88.2,143.4 124.4,165.1 160.5,175.4 196.7,143.1 232.9,138.3 269.1,132.0 305.3,112.8 341.5,108.0 377.6,86.0 413.8,64.8 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.02 ns | 2.3 µs | 4.26 µs | 4.47 µs | 2.66 µs |
| D38 | 6.54 ns | 4.26 µs | 4.45 µs | 4.05 µs | 5.95 µs |
| D57 | 62.4 ns | 4.58 µs | 4.52 µs | 4.69 µs | 5.23 µs |
| D76 | 74.7 ns | 4.49 µs | 5.05 µs | 5.13 µs | 4.13 µs |
| D115 | 138 ns | 8.31 µs | 9.07 µs | 11.2 µs | 10.9 µs |
| D153 | 198 ns | 8.67 µs | 8.74 µs | 12 µs | 13.2 µs |
| D230 | 350 ns | 13.8 µs | 15.2 µs | 13.7 µs | 18.4 µs |
| D307 | 365 ns | 19.5 µs | 18.7 µs | 28.5 µs | 35 µs |
| D462 | 673 ns | 77.4 µs | 122 µs | 206 µs | 283 µs |
| D616 | 802 ns | 177 µs | 193 µs | 353 µs | 531 µs |
| D924 | 958 ns | 453 µs | 488 µs | 849 µs | 1.61 ms |
| D1232 | 1.53 µs | 646 µs | 855 µs | 2.11 ms | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.7 88.2,186.7 124.4,158.7 160.5,156.5 196.7,148.9 232.9,144.4 269.1,137.3 305.3,136.8 341.5,129.2 377.6,127.0 413.8,124.8 450.0,119.0 450.0,26.4 413.8,32.7 377.6,46.4 341.5,54.2 305.3,80.2 269.1,88.1 232.9,92.3 196.7,94.7 160.5,106.7 124.4,103.8 88.2,102.2 52.0,112.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.7 88.2,186.7 124.4,158.7 160.5,156.5 196.7,148.9 232.9,144.4 269.1,137.3 305.3,136.8 341.5,129.2 377.6,127.0 413.8,124.8 450.0,119.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,113.9 88.2,106.3 124.4,105.4 160.5,105.6 196.7,98.0 232.9,97.5 269.1,91.8 305.3,87.5 341.5,70.3 377.6,60.1 413.8,48.4 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,105.7 124.4,105.6 160.5,104.2 196.7,96.9 232.9,97.4 269.1,90.5 305.3,88.0 341.5,64.7 377.6,59.0 413.8,47.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.7 88.2,106.9 124.4,105.1 160.5,104.0 196.7,94.3 232.9,93.5 269.1,91.8 305.3,82.7 341.5,58.2 377.6,51.5 413.8,40.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,102.2 124.4,103.8 160.5,106.7 196.7,94.7 232.9,92.3 269.1,88.1 305.3,80.2 341.5,54.2 377.6,46.4 413.8,32.7 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.93 ns | 13.6 ns | 18.5 ns | 29.7 ns | 21.3 ns |
| D38 | 7.93 ns | 20.9 ns | 31.3 ns | 246 ns | 489 ns |
| D57 | 176 ns | 223 ns | 491 ns | 771 ns | 770 ns |
| D76 | 192 ns | 279 ns | 640 ns | 830 ns | 749 ns |
| D115 | 115 ns | 649 ns | 991 ns | 1.51 µs | 1.68 µs |
| D153 | 139 ns | 1.1 µs | 1.19 µs | 1.83 µs | 2.69 µs |
| D230 | 158 ns | 1.69 µs | 2.38 µs | 2.47 µs | 3.64 µs |
| D307 | 144 ns | 2.43 µs | 3.11 µs | 4.76 µs | 7.18 µs |
| D462 | 179 ns | 3.88 µs | 5.06 µs | 9.33 µs | 11.5 µs |
| D616 | 246 ns | 5.95 µs | 6.31 µs | 14.4 µs | 20.2 µs |
| D924 | 226 ns | 11.4 µs | 17.2 µs | 26.6 µs | 35 µs |
| D1232 | 303 ns | 13 µs | 28.4 µs | 39.2 µs | 60.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.1 88.2,174.0 124.4,120.1 160.5,118.6 196.7,127.6 232.9,124.3 269.1,122.0 305.3,123.7 341.5,119.8 377.6,114.4 413.8,115.8 450.0,110.8 450.0,18.7 413.8,28.3 377.6,37.8 341.5,47.5 305.3,55.8 269.1,67.6 232.9,72.8 196.7,81.0 160.5,95.0 124.4,94.5 88.2,102.4 52.0,156.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.1 88.2,174.0 124.4,120.1 160.5,118.6 196.7,127.6 232.9,124.3 269.1,122.0 305.3,123.7 341.5,119.8 377.6,114.4 413.8,115.8 450.0,110.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.7 88.2,157.2 124.4,116.1 160.5,112.2 196.7,97.5 232.9,88.3 269.1,80.8 305.3,74.6 341.5,66.5 377.6,59.0 413.8,47.8 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,150.2 124.4,102.4 160.5,97.8 196.7,90.1 232.9,86.9 269.1,74.9 305.3,70.3 341.5,61.8 377.6,58.0 413.8,40.6 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,114.3 124.4,94.5 160.5,93.2 196.7,82.9 232.9,79.5 269.1,74.3 305.3,62.9 341.5,51.2 377.6,43.6 413.8,33.0 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.8 88.2,102.4 124.4,94.5 160.5,95.0 196.7,81.0 232.9,72.8 269.1,67.6 305.3,55.8 341.5,47.5 377.6,37.8 413.8,28.3 450.0,18.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
