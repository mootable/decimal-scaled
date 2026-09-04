# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 78 ns | 131 ns | 159 ns | 258 ns | 205 ns |
| D38 | 64.5 ns | 155 ns | 263 ns | 372 ns | 395 ns |
| D57 | 189 ns | 428 ns | 645 ns | 1.05 µs | 798 ns |
| D76 | 200 ns | 489 ns | 803 ns | 1.27 µs | 1.44 µs |
| D115 | 228 ns | 770 ns | 1.33 µs | 2.02 µs | 2.99 µs |
| D153 | 251 ns | 737 ns | 1.35 µs | 2.94 µs | 4.52 µs |
| D230 | 278 ns | 1.43 µs | 3.26 µs | 3.63 µs | 7.36 µs |
| D307 | 329 ns | 1.51 µs | 3.16 µs | 9.63 µs | 12.7 µs |
| D462 | 379 ns | 2.77 µs | 8.89 µs | 14.3 µs | 30.1 µs |
| D616 | 453 ns | 3.69 µs | 14.4 µs | 34.1 µs | 56.2 µs |
| D924 | 542 ns | 6.07 µs | 27.4 µs | 54.5 µs | 101 µs |
| D1232 | 1.34 µs | 10 µs | 57.6 µs | 103 µs | 207 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.3 88.2,177.6 124.4,158.9 160.5,157.9 196.7,155.7 232.9,154.0 269.1,152.3 305.3,149.3 341.5,146.9 377.6,143.8 413.8,140.6 450.0,124.9 450.0,37.4 413.8,49.8 377.6,60.0 341.5,70.9 305.3,85.8 269.1,95.3 232.9,103.8 196.7,111.0 160.5,123.6 124.4,133.9 88.2,146.1 52.0,157.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.3 88.2,177.6 124.4,158.9 160.5,157.9 196.7,155.7 232.9,154.0 269.1,152.3 305.3,149.3 341.5,146.9 377.6,143.8 413.8,140.6 450.0,124.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.4 88.2,162.4 124.4,144.7 160.5,142.4 196.7,134.5 232.9,135.3 269.1,123.8 305.3,122.8 341.5,112.3 377.6,107.3 413.8,98.7 450.0,90.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.9 88.2,153.2 124.4,137.6 160.5,133.8 196.7,125.1 232.9,124.8 269.1,109.5 305.3,110.0 341.5,92.0 377.6,83.6 413.8,72.5 450.0,59.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,147.2 124.4,129.2 160.5,125.8 196.7,117.8 232.9,111.3 269.1,107.6 305.3,90.6 341.5,83.8 377.6,68.7 413.8,60.5 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.5 88.2,146.1 124.4,133.9 160.5,123.6 196.7,111.0 232.9,103.8 269.1,95.3 305.3,85.8 341.5,70.9 377.6,60.0 413.8,49.8 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.62 µs | 1.63 µs | 3.5 µs | 3.43 µs | 3.17 µs |
| D38 | 1.29 µs | 3.24 µs | 4.21 µs | 4.34 µs | 3.98 µs |
| D57 | 3.81 µs | 3.87 µs | 4.8 µs | 6.14 µs | 7.38 µs |
| D76 | 5.01 µs | 6.41 µs | 7.01 µs | 9.27 µs | 9.21 µs |
| D115 | 4.64 µs | 6.75 µs | 13.2 µs | 18.3 µs | 22.3 µs |
| D153 | 5.76 µs | 6 µs | 11.5 µs | 19.2 µs | 37.8 µs |
| D230 | 4.64 µs | 13.2 µs | 23.6 µs | 28.3 µs | 67.7 µs |
| D307 | 5.77 µs | 13.7 µs | 23.4 µs | 83.9 µs | 101 µs |
| D462 | 5.28 µs | 19.9 µs | 80.2 µs | 130 µs | 230 µs |
| D616 | 5.39 µs | 27.9 µs | 133 µs | 285 µs | 445 µs |
| D924 | 5.61 µs | 51.4 µs | 229 µs | 472 µs | 773 µs |
| D1232 | 5.84 µs | 85.4 µs | 444 µs | 772 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.6 88.2,204.5 124.4,181.0 160.5,175.0 196.7,176.7 232.9,172.0 269.1,176.7 305.3,171.9 341.5,173.9 377.6,173.4 413.8,172.6 450.0,171.7 450.0,37.4 413.8,65.6 377.6,77.6 341.5,91.9 305.3,109.8 269.1,118.5 232.9,131.1 196.7,142.6 160.5,161.8 124.4,166.6 88.2,180.0 52.0,184.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.6 88.2,204.5 124.4,181.0 160.5,175.0 196.7,176.7 232.9,172.0 269.1,176.7 305.3,171.9 341.5,173.9 377.6,173.4 413.8,172.6 450.0,171.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.3 88.2,184.5 124.4,180.6 160.5,169.7 196.7,168.5 232.9,171.1 269.1,153.9 305.3,153.2 341.5,145.0 377.6,137.7 413.8,124.5 450.0,113.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.8 88.2,178.8 124.4,175.9 160.5,167.7 196.7,153.9 232.9,157.0 269.1,141.4 305.3,141.5 341.5,114.8 377.6,103.8 413.8,92.1 450.0,77.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.2 88.2,178.2 124.4,170.6 160.5,161.6 196.7,146.9 232.9,145.8 269.1,137.4 305.3,113.8 341.5,104.3 377.6,87.3 413.8,76.3 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.9 88.2,180.0 124.4,166.6 160.5,161.8 196.7,142.6 232.9,131.1 269.1,118.5 305.3,109.8 341.5,91.9 377.6,77.6 413.8,65.6 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 40.5 ns | 42.9 ns | 41.1 ns | 31.1 ns |
| D38 | 12 ns | 41.1 ns | 74.4 ns | 67.7 ns | 97.6 ns |
| D57 | 16.6 ns | 41.4 ns | 95.9 ns | 402 ns | 352 ns |
| D76 | 18.2 ns | 96.7 ns | 368 ns | 380 ns | 551 ns |
| D115 | 16.4 ns | 83.8 ns | 393 ns | 809 ns | 955 ns |
| D153 | 23.1 ns | 330 ns | 502 ns | 874 ns | 1.74 µs |
| D230 | 22.4 ns | 423 ns | 1.14 µs | 1.27 µs | 2.28 µs |
| D307 | 43.1 ns | 603 ns | 1.18 µs | 2.97 µs | 4.83 µs |
| D462 | 61.8 ns | 973 ns | 3.14 µs | 4.77 µs | 7.28 µs |
| D616 | 69.2 ns | 1.57 µs | 5.68 µs | 10.9 µs | 13.9 µs |
| D924 | 99.3 ns | 2.22 µs | 9.07 µs | 19.1 µs | 21 µs |
| D1232 | 100 ns | 4.03 µs | 20.5 µs | 20.3 µs | 45.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,206.0 124.4,199.0 160.5,197.0 196.7,199.3 232.9,191.8 269.1,192.5 305.3,178.3 341.5,170.5 377.6,168.0 413.8,160.1 450.0,160.0 450.0,27.0 413.8,43.9 377.6,52.9 341.5,66.9 305.3,75.8 269.1,92.1 232.9,97.9 196.7,111.0 160.5,122.9 124.4,132.7 88.2,160.5 52.0,185.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,206.0 124.4,199.0 160.5,197.0 196.7,199.3 232.9,191.8 269.1,192.5 305.3,178.3 341.5,170.5 377.6,168.0 413.8,160.1 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.3 124.4,179.2 160.5,160.7 196.7,163.8 232.9,134.1 269.1,128.7 305.3,121.0 341.5,110.6 377.6,100.2 413.8,92.7 450.0,79.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,166.4 124.4,160.9 160.5,131.7 196.7,130.3 232.9,125.0 269.1,107.2 305.3,106.3 341.5,85.2 377.6,72.3 413.8,62.1 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.3 88.2,168.5 124.4,129.8 160.5,131.0 196.7,114.6 232.9,112.9 269.1,104.8 305.3,86.4 341.5,76.1 377.6,58.1 413.8,46.0 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.4 88.2,160.5 124.4,132.7 160.5,122.9 196.7,111.0 232.9,97.9 269.1,92.1 305.3,75.8 341.5,66.9 377.6,52.9 413.8,43.9 450.0,27.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.62 ns | 276 ns | 401 ns | 384 ns | 331 ns |
| D38 | 7.4 ns | 371 ns | 415 ns | 380 ns | 379 ns |
| D57 | 278 ns | 474 ns | 508 ns | 525 ns | 443 ns |
| D76 | 276 ns | 522 ns | 489 ns | 638 ns | 465 ns |
| D115 | 263 ns | 509 ns | 631 ns | 947 ns | 1 µs |
| D153 | 359 ns | 454 ns | 449 ns | 847 ns | 1.4 µs |
| D230 | 470 ns | 654 ns | 1.18 µs | 941 ns | 1.66 µs |
| D307 | 751 ns | 637 ns | 754 ns | 1.48 µs | 8.84 µs |
| D462 | 1.21 µs | 2.64 µs | 3.37 µs | 3.5 µs | 5.02 µs |
| D616 | 1.42 µs | 1.28 µs | 2 µs | 3.01 µs | 3.94 µs |
| D924 | 2.03 µs | 1.32 µs | 2.34 µs | 3.34 µs | 4.27 µs |
| D1232 | 3.11 µs | 1.94 µs | 4.56 µs | 4.41 µs | 6.93 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,172.5 88.2,166.5 124.4,87.8 160.5,88.0 196.7,89.0 232.9,82.3 269.1,76.4 305.3,66.2 341.5,55.8 377.6,52.4 413.8,44.6 450.0,35.4 450.0,18.0 413.8,28.5 377.6,30.2 341.5,25.0 305.3,12.7 269.1,49.0 232.9,52.7 196.7,59.9 160.5,76.6 124.4,77.7 88.2,81.1 52.0,84.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,172.5 88.2,166.5 124.4,87.8 160.5,88.0 196.7,89.0 232.9,82.3 269.1,76.4 305.3,66.2 341.5,55.8 377.6,52.4 413.8,44.6 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,87.9 88.2,81.5 124.4,76.2 160.5,74.1 196.7,74.7 232.9,77.1 269.1,69.2 305.3,69.8 341.5,38.9 377.6,54.6 413.8,53.9 450.0,45.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,79.1 124.4,74.7 160.5,75.5 196.7,70.0 232.9,77.4 269.1,56.3 305.3,66.1 341.5,33.6 377.6,44.9 413.8,41.6 450.0,27.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.8 88.2,81.0 124.4,74.0 160.5,69.8 196.7,61.2 232.9,63.6 269.1,61.3 305.3,51.4 341.5,32.8 377.6,36.0 413.8,33.8 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,84.0 88.2,81.1 124.4,77.7 160.5,76.6 196.7,59.9 232.9,52.7 269.1,49.0 305.3,12.7 341.5,25.0 377.6,30.2 413.8,28.5 450.0,18.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.89 µs | 3.13 µs | 6.08 µs | 6.33 µs | 5.9 µs |
| D38 | 2.35 µs | 5.85 µs | 7.67 µs | 8.66 µs | 8.83 µs |
| D57 | 2.51 µs | 2.59 µs | 2.94 µs | 2.95 µs | 2.2 µs |
| D76 | 2.5 µs | 2.94 µs | 2.68 µs | 2.83 µs | 2.23 µs |
| D115 | 3.77 µs | 5.05 µs | 5.27 µs | 5.6 µs | 5.91 µs |
| D153 | 4.85 µs | 4.62 µs | 4.17 µs | 4.87 µs | 6.86 µs |
| D230 | 5.64 µs | 7.28 µs | 8.93 µs | 5.68 µs | 8.87 µs |
| D307 | 9.88 µs | 9.15 µs | 8.12 µs | 15.3 µs | 13.4 µs |
| D462 | 9.66 µs | 10.3 µs | 14.1 µs | 13.8 µs | 18.4 µs |
| D616 | 16.2 µs | 17.7 µs | 30.1 µs | 39.7 µs | 46 µs |
| D924 | 24 µs | 26.1 µs | 48.1 µs | 60.7 µs | 73.1 µs |
| D1232 | 32.7 µs | 41.1 µs | 97.3 µs | 99.5 µs | 142 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.3 88.2,185.3 124.4,183.3 160.5,183.4 196.7,171.6 232.9,164.3 269.1,159.9 305.3,143.7 341.5,144.3 377.6,129.4 413.8,118.0 450.0,109.0 450.0,66.6 413.8,85.7 377.6,99.2 341.5,125.6 305.3,134.9 269.1,146.8 232.9,154.3 196.7,158.6 160.5,186.8 124.4,187.2 88.2,146.9 52.0,158.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.3 88.2,185.3 124.4,183.3 160.5,183.4 196.7,171.6 232.9,164.3 269.1,159.9 305.3,143.7 341.5,144.3 377.6,129.4 413.8,118.0 450.0,109.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.9 88.2,158.8 124.4,182.5 160.5,178.7 196.7,163.1 232.9,165.7 269.1,152.5 305.3,145.9 341.5,142.4 377.6,126.8 413.8,115.5 450.0,102.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.8 88.2,151.0 124.4,178.8 160.5,181.4 196.7,161.9 232.9,168.6 269.1,146.6 305.3,149.3 341.5,133.4 377.6,111.4 413.8,97.9 450.0,77.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.6 88.2,147.5 124.4,178.7 160.5,179.8 196.7,160.1 232.9,164.2 269.1,159.7 305.3,131.1 341.5,134.1 377.6,103.4 413.8,91.1 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,146.9 124.4,187.2 160.5,186.8 196.7,158.6 232.9,154.3 269.1,146.8 305.3,134.9 341.5,125.6 377.6,99.2 413.8,85.7 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 407 ns | 392 ns | 1.36 µs | 1.36 µs | 1.23 µs |
| D38 | 321 ns | 1.29 µs | 1.62 µs | 1.71 µs | 1.14 µs |
| D57 | 1.24 µs | 1.32 µs | 1.47 µs | 1.56 µs | 1.11 µs |
| D76 | 1.22 µs | 1.48 µs | 1.36 µs | 1.46 µs | 1.14 µs |
| D115 | 1.94 µs | 2.45 µs | 2.54 µs | 2.79 µs | 2.9 µs |
| D153 | 2.42 µs | 2.3 µs | 2.17 µs | 2.52 µs | 3.27 µs |
| D230 | 2.82 µs | 3.4 µs | 4.25 µs | 2.82 µs | 4.14 µs |
| D307 | 4.93 µs | 4.4 µs | 3.88 µs | 6.87 µs | 6.17 µs |
| D462 | 4.87 µs | 4.88 µs | 6.44 µs | 6.4 µs | 8.51 µs |
| D616 | 8.04 µs | 7.57 µs | 12.3 µs | 15.3 µs | 18.2 µs |
| D924 | 11.6 µs | 10.6 µs | 17.6 µs | 22.2 µs | 26.3 µs |
| D1232 | 16 µs | 16 µs | 34 µs | 34.6 µs | 49.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,176.2 124.4,137.1 160.5,137.5 196.7,124.2 232.9,117.8 269.1,113.3 305.3,97.2 341.5,97.5 377.6,83.0 413.8,72.3 450.0,63.1 450.0,30.4 413.8,48.7 377.6,59.3 341.5,81.3 305.3,90.7 269.1,102.2 232.9,109.0 196.7,112.5 160.5,139.6 124.4,140.3 88.2,139.5 52.0,137.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,176.2 124.4,137.1 160.5,137.5 196.7,124.2 232.9,117.8 269.1,113.3 305.3,97.2 341.5,97.5 377.6,83.0 413.8,72.3 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.5 88.2,136.0 124.4,135.2 160.5,132.0 196.7,117.4 232.9,119.2 269.1,107.9 305.3,100.5 341.5,97.5 377.6,84.7 413.8,74.9 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,129.4 124.4,132.3 160.5,134.4 196.7,116.4 232.9,120.9 269.1,101.4 305.3,104.0 341.5,89.4 377.6,70.7 413.8,60.4 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,127.7 124.4,130.5 160.5,132.3 196.7,113.6 232.9,116.6 269.1,113.4 305.3,87.5 341.5,89.6 377.6,64.4 413.8,53.6 450.0,40.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.3 88.2,139.5 124.4,140.3 160.5,139.6 196.7,112.5 232.9,109.0 269.1,102.2 305.3,90.7 341.5,81.3 377.6,59.3 413.8,48.7 450.0,30.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 366 ns | 347 ns | 1.34 µs | 1.32 µs | 1.21 µs |
| D38 | 293 ns | 1.26 µs | 1.59 µs | 1.69 µs | 1.1 µs |
| D57 | 993 ns | 1.01 µs | 1.4 µs | 1.38 µs | 991 ns |
| D76 | 941 ns | 1.18 µs | 1.29 µs | 1.42 µs | 1.01 µs |
| D115 | 1.49 µs | 2.23 µs | 2.27 µs | 2.47 µs | 2.58 µs |
| D153 | 1.91 µs | 1.89 µs | 1.75 µs | 2.08 µs | 2.82 µs |
| D230 | 2.13 µs | 2.89 µs | 3.49 µs | 2.31 µs | 3.51 µs |
| D307 | 4.07 µs | 3.79 µs | 3.29 µs | 6.01 µs | 5.42 µs |
| D462 | 3.85 µs | 4.17 µs | 5.75 µs | 5.56 µs | 7.58 µs |
| D616 | 6.28 µs | 6.2 µs | 10.5 µs | 13.3 µs | 16 µs |
| D924 | 8.94 µs | 8.7 µs | 15.2 µs | 19.6 µs | 23.8 µs |
| D1232 | 12.2 µs | 13.8 µs | 30.5 µs | 31.3 µs | 46.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,172.4 88.2,178.9 124.4,143.5 160.5,145.1 196.7,131.8 232.9,124.6 269.1,121.4 305.3,102.7 341.5,104.3 377.6,90.1 413.8,79.9 450.0,70.8 450.0,32.4 413.8,51.5 377.6,63.0 341.5,84.7 305.3,94.4 269.1,107.0 232.9,113.3 196.7,115.9 160.5,143.0 124.4,143.6 88.2,140.4 52.0,137.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,172.4 88.2,178.9 124.4,143.5 160.5,145.1 196.7,131.8 232.9,124.6 269.1,121.4 305.3,102.7 341.5,104.3 377.6,90.1 413.8,79.9 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.0 88.2,136.5 124.4,142.9 160.5,138.6 196.7,120.1 232.9,124.9 269.1,112.6 305.3,104.8 341.5,102.0 377.6,90.5 413.8,80.7 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.8 88.2,129.9 124.4,133.6 160.5,136.0 196.7,119.6 232.9,127.2 269.1,107.2 305.3,108.9 341.5,92.7 377.6,75.1 413.8,64.5 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,135.3 88.2,128.1 124.4,134.1 160.5,133.3 196.7,117.2 232.9,122.2 269.1,119.1 305.3,91.4 341.5,93.7 377.6,68.4 413.8,57.2 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.9 88.2,140.4 124.4,143.6 160.5,143.0 196.7,115.9 232.9,113.3 269.1,107.0 305.3,94.4 341.5,84.7 377.6,63.0 413.8,51.5 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 2.05 µs | 4.19 µs | 4.04 µs | 3.72 µs |
| D38 | 10.2 ns | 3.87 µs | 4.85 µs | 4.97 µs | 4.62 µs |
| D57 | 58.6 ns | 4.19 µs | 4.88 µs | 5.1 µs | 4.06 µs |
| D76 | 68.7 ns | 4.51 µs | 4.62 µs | 5.12 µs | 4.44 µs |
| D115 | 113 ns | 8.19 µs | 9.05 µs | 10.2 µs | 10.9 µs |
| D153 | 193 ns | 7.43 µs | 7.59 µs | 9.22 µs | 12.9 µs |
| D230 | 277 ns | 12.5 µs | 16.3 µs | 11.9 µs | 18.6 µs |
| D307 | 469 ns | 16 µs | 13.8 µs | 30.3 µs | 28.2 µs |
| D462 | 809 ns | 61.1 µs | 137 µs | 173 µs | 262 µs |
| D616 | 786 ns | 140 µs | 328 µs | 380 µs | 570 µs |
| D924 | 1.01 µs | 286 µs | 386 µs | 664 µs | 1.36 ms |
| D1232 | 1.61 µs | 547 µs | 867 µs | 1.8 ms | 2.75 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.5 88.2,209.7 124.4,184.4 160.5,182.1 196.7,174.8 232.9,167.1 269.1,161.9 305.3,154.3 341.5,146.4 377.6,146.8 413.8,143.2 450.0,136.4 450.0,28.7 413.8,38.9 377.6,51.5 341.5,62.7 305.3,95.0 269.1,101.0 232.9,106.3 196.7,108.8 160.5,121.8 124.4,123.1 88.2,121.2 52.0,124.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.5 88.2,209.7 124.4,184.4 160.5,182.1 196.7,174.8 232.9,167.1 269.1,161.9 305.3,154.3 341.5,146.4 377.6,146.8 413.8,143.2 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,132.9 88.2,123.7 124.4,122.6 160.5,121.5 196.7,112.9 232.9,114.3 269.1,106.7 305.3,103.2 341.5,83.8 377.6,71.8 413.8,61.4 450.0,52.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.6 88.2,120.5 124.4,120.4 160.5,121.2 196.7,111.5 232.9,114.0 269.1,102.9 305.3,105.3 341.5,72.2 377.6,59.5 413.8,57.1 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.1 88.2,120.1 124.4,119.8 160.5,119.7 196.7,109.7 232.9,111.2 269.1,107.4 305.3,94.0 341.5,68.7 377.6,57.4 413.8,49.3 450.0,34.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.3 88.2,121.2 124.4,123.1 160.5,121.8 196.7,108.8 232.9,106.3 269.1,101.0 305.3,95.0 341.5,62.7 377.6,51.5 413.8,38.9 450.0,28.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.18 ns | 13.6 ns | 18.5 ns | 28.6 ns | 22.8 ns |
| D38 | 5.64 ns | 20.1 ns | 33.3 ns | 135 ns | 193 ns |
| D57 | 78.4 ns | 141 ns | 219 ns | 390 ns | 330 ns |
| D76 | 83 ns | 144 ns | 287 ns | 373 ns | 538 ns |
| D115 | 103 ns | 226 ns | 384 ns | 741 ns | 923 ns |
| D153 | 110 ns | 270 ns | 486 ns | 850 ns | 1.65 µs |
| D230 | 131 ns | 423 ns | 1.03 µs | 1.3 µs | 2.22 µs |
| D307 | 172 ns | 571 ns | 1.15 µs | 2.89 µs | 4.65 µs |
| D462 | 204 ns | 918 ns | 2.61 µs | 4.86 µs | 6.98 µs |
| D616 | 233 ns | 1.52 µs | 4.67 µs | 7.39 µs | 13.5 µs |
| D924 | 251 ns | 1.78 µs | 6.4 µs | 12.7 µs | 20.5 µs |
| D1232 | 396 ns | 3.35 µs | 13.8 µs | 20.6 µs | 45.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,180.0 124.4,134.2 160.5,133.2 196.7,129.5 232.9,128.4 269.1,125.3 305.3,120.6 341.5,117.6 377.6,115.3 413.8,114.0 450.0,106.1 450.0,23.8 413.8,37.6 377.6,44.8 341.5,56.2 305.3,63.3 269.1,76.2 232.9,81.3 196.7,91.4 160.5,100.8 124.4,109.2 88.2,118.5 52.0,155.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,180.0 124.4,134.2 160.5,133.2 196.7,129.5 232.9,128.4 269.1,125.3 305.3,120.6 341.5,117.6 377.6,115.3 413.8,114.0 450.0,106.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.7 88.2,157.8 124.4,124.1 160.5,123.7 196.7,115.8 232.9,112.8 269.1,104.9 305.3,99.7 341.5,91.5 377.6,82.8 413.8,80.0 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,149.1 124.4,116.4 160.5,111.7 196.7,106.6 232.9,102.6 269.1,89.5 305.3,87.6 341.5,73.3 377.6,63.2 413.8,57.8 450.0,44.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,124.8 124.4,106.3 160.5,107.1 196.7,95.2 232.9,92.8 269.1,85.5 305.3,71.5 341.5,62.5 377.6,55.3 413.8,45.8 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.7 88.2,118.5 124.4,109.2 160.5,100.8 196.7,91.4 232.9,81.3 269.1,76.2 305.3,63.3 341.5,56.2 377.6,44.8 413.8,37.6 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
