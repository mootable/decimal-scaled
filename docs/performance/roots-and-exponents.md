# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 311 ns | 303 ns | 141 ns | 228 ns | 386 ns |
| D38 | 262 ns | 264 ns | 532 ns | 599 ns | 841 ns |
| D57 | 341 ns | 527 ns | 707 ns | 1.15 µs | 622 ns |
| D76 | 398 ns | 746 ns | 1.18 µs | 1.02 µs | 1.89 µs |
| D115 | 336 ns | 2.29 µs | 2.54 µs | 3.88 µs | 5.22 µs |
| D153 | 357 ns | 2.66 µs | 3.48 µs | 5.67 µs | 6.79 µs |
| D230 | 397 ns | 3.7 µs | 5.23 µs | 9.77 µs | 13.8 µs |
| D307 | 570 ns | 4.79 µs | 11.7 µs | 14.3 µs | 20.7 µs |
| D462 | 368 ns | 5.88 µs | 26.6 µs | 33.2 µs | 51.6 µs |
| D616 | 540 ns | 16.9 µs | 37.1 µs | 57.8 µs | 84.6 µs |
| D924 | 414 ns | 30.2 µs | 89.6 µs | 108 µs | 198 µs |
| D1232 | 702 ns | 55.2 µs | 138 µs | 194 µs | 313 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,189.1 124.4,183.4 160.5,180.0 196.7,183.7 232.9,182.4 269.1,180.1 305.3,172.2 341.5,181.7 377.6,173.4 413.8,179.2 450.0,167.7 450.0,35.3 413.8,45.1 377.6,63.6 341.5,74.4 305.3,94.2 269.1,102.9 232.9,118.4 196.7,124.1 160.5,146.2 124.4,170.3 88.2,163.8 52.0,180.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,189.1 124.4,183.4 160.5,180.0 196.7,183.7 232.9,182.4 269.1,180.1 305.3,172.2 341.5,181.7 377.6,173.4 413.8,179.2 450.0,167.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,185.9 88.2,188.9 124.4,173.9 160.5,166.4 196.7,142.0 232.9,138.8 269.1,131.6 305.3,126.0 341.5,121.5 377.6,98.6 413.8,86.0 450.0,72.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,202.5 88.2,173.7 124.4,167.5 160.5,156.4 196.7,139.8 232.9,132.9 269.1,124.1 305.3,106.5 341.5,88.7 377.6,81.5 413.8,62.4 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.1 88.2,171.1 124.4,157.0 160.5,159.6 196.7,130.5 232.9,122.3 269.1,110.5 305.3,102.3 341.5,84.0 377.6,71.9 413.8,58.3 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.7 88.2,163.8 124.4,170.3 160.5,146.2 196.7,124.1 232.9,118.4 269.1,102.9 305.3,94.2 341.5,74.4 377.6,63.6 413.8,45.1 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.21 µs | 2.29 µs | 2.82 µs | 3.69 µs |
| D38 | 1.24 ns | 3.24 µs | 3.82 µs | 4.29 µs | 4.74 µs |
| D57 | 2.81 ns | 4.18 µs | 4.42 µs | 5.8 µs | 5.51 µs |
| D76 | 2.73 ns | 6.44 µs | 7 µs | 7.68 µs | 11.1 µs |
| D115 | 13.2 ns | 6.61 µs | 14 µs | 19.1 µs | 23.7 µs |
| D153 | 18 ns | 7.21 µs | 16.1 µs | 23.5 µs | 38.1 µs |
| D230 | 45.4 ns | 11.7 µs | 15.1 µs | 42.4 µs | 78.6 µs |
| D307 | 68.1 ns | 14.4 µs | 38.6 µs | 73.8 µs | 116 µs |
| D462 | 108 ns | 13.5 µs | 84.9 µs | 154 µs | 246 µs |
| D616 | 134 ns | 39.4 µs | 131 µs | 267 µs | 444 µs |
| D924 | 122 ns | 80.9 µs | 288 µs | 548 µs | 988 µs |
| D1232 | 294 ns | 130 µs | 407 µs | 771 µs | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,207.4 124.4,197.2 160.5,197.5 196.7,177.9 232.9,174.1 269.1,162.7 305.3,157.6 341.5,151.9 377.6,149.2 413.8,150.4 450.0,139.5 450.0,25.0 413.8,38.7 377.6,48.6 341.5,56.0 305.3,65.3 269.1,70.1 232.9,79.1 196.7,85.0 160.5,94.4 124.4,103.1 88.2,105.0 52.0,108.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,207.4 124.4,197.2 160.5,197.5 196.7,177.9 232.9,174.1 269.1,162.7 305.3,157.6 341.5,151.9 377.6,149.2 413.8,150.4 450.0,139.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,109.7 124.4,106.5 160.5,101.2 196.7,100.9 232.9,99.8 269.1,93.8 305.3,91.2 341.5,92.0 377.6,78.7 413.8,69.8 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.0 88.2,107.7 124.4,105.8 160.5,100.1 196.7,91.6 232.9,89.8 269.1,90.6 305.3,79.0 341.5,69.2 377.6,63.8 413.8,54.0 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,106.2 124.4,102.5 160.5,99.0 196.7,87.7 232.9,85.1 269.1,77.8 305.3,70.9 341.5,61.8 377.6,55.0 413.8,46.0 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,105.0 124.4,103.1 160.5,94.4 196.7,85.0 232.9,79.1 269.1,70.1 305.3,65.3 341.5,56.0 377.6,48.6 413.8,38.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 27.2 ns | 33.2 ns | 36.6 ns | 41.4 ns |
| D38 | 12 ns | 41.4 ns | 66.4 ns | 66.5 ns | 94.4 ns |
| D57 | 16.4 ns | 41.5 ns | 67.9 ns | 692 ns | 346 ns |
| D76 | 14.8 ns | 73.9 ns | 691 ns | 434 ns | 1.08 µs |
| D115 | 19.8 ns | 72.8 ns | 621 ns | 1.08 µs | 1.28 µs |
| D153 | 23.4 ns | 710 ns | 1.1 µs | 1.32 µs | 1.99 µs |
| D230 | 28.6 ns | 507 ns | 824 ns | 2.39 µs | 3.3 µs |
| D307 | 44.1 ns | 812 ns | 2.05 µs | 2.95 µs | 5.49 µs |
| D462 | 63.5 ns | 809 ns | 3.59 µs | 6.31 µs | 9.08 µs |
| D616 | 72.8 ns | 2.41 µs | 6.09 µs | 10.9 µs | 15.3 µs |
| D924 | 78.4 ns | 3.75 µs | 11.3 µs | 21.6 µs | 28.5 µs |
| D1232 | 90.1 ns | 6.15 µs | 19.7 µs | 21.1 µs | 49.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,206.1 124.4,199.3 160.5,201.5 196.7,195.1 232.9,191.5 269.1,187.2 305.3,177.8 341.5,169.9 377.6,166.9 413.8,165.3 450.0,162.3 450.0,25.1 413.8,37.3 377.6,50.7 341.5,62.1 305.3,73.0 269.1,84.1 232.9,95.0 196.7,104.7 160.5,108.3 124.4,133.0 88.2,161.3 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,206.1 124.4,199.3 160.5,201.5 196.7,195.1 232.9,191.5 269.1,187.2 305.3,177.8 341.5,169.9 377.6,166.9 413.8,165.3 450.0,162.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.3 88.2,179.1 124.4,179.1 160.5,166.6 196.7,166.9 232.9,117.4 269.1,124.7 305.3,114.5 341.5,114.6 377.6,90.9 413.8,81.3 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.0 88.2,168.9 124.4,168.4 160.5,118.0 196.7,120.3 232.9,107.9 269.1,114.2 305.3,94.4 341.5,82.2 377.6,70.8 413.8,57.3 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.8 88.2,168.9 124.4,118.0 160.5,128.1 196.7,108.3 232.9,104.0 269.1,91.1 305.3,86.5 341.5,70.0 377.6,58.1 413.8,43.2 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,161.3 124.4,133.0 160.5,108.3 196.7,104.7 232.9,95.0 269.1,84.1 305.3,73.0 341.5,62.1 377.6,50.7 413.8,37.3 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.94 ns | 198 ns | 302 ns | 353 ns | 386 ns |
| D38 | 2.18 ns | 380 ns | 390 ns | 385 ns | 399 ns |
| D57 | 281 ns | 506 ns | 481 ns | 487 ns | 345 ns |
| D76 | 249 ns | 502 ns | 465 ns | 460 ns | 621 ns |
| D115 | 317 ns | 491 ns | 720 ns | 1.07 µs | 1.11 µs |
| D153 | 354 ns | 525 ns | 636 ns | 1.13 µs | 1.42 µs |
| D230 | 573 ns | 554 ns | 739 ns | 1.31 µs | 1.84 µs |
| D307 | 652 ns | 595 ns | 1.22 µs | 1.2 µs | 10.5 µs |
| D462 | 1.06 µs | 1.7 µs | 3.68 µs | 4 µs | 5.67 µs |
| D616 | 1.58 µs | 1.58 µs | 1.88 µs | 2.92 µs | 3.92 µs |
| D924 | 1.64 µs | 2.1 µs | 3.15 µs | 3.12 µs | 5.27 µs |
| D1232 | 2.58 µs | 3.11 µs | 4.25 µs | 4.31 µs | 7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.2 88.2,196.4 124.4,112.1 160.5,114.2 196.7,110.0 232.9,108.0 269.1,99.7 305.3,97.4 341.5,89.1 377.6,82.0 413.8,81.4 450.0,73.6 450.0,56.2 413.8,61.1 377.6,66.3 341.5,59.8 305.3,49.1 269.1,79.4 232.9,84.0 196.7,88.2 160.5,98.3 124.4,108.5 88.2,106.0 52.0,106.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.2 88.2,196.4 124.4,112.1 160.5,114.2 196.7,110.0 232.9,108.0 269.1,99.7 305.3,97.4 341.5,89.1 377.6,82.0 413.8,81.4 450.0,73.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,106.8 124.4,101.8 160.5,102.0 196.7,102.3 232.9,101.2 269.1,100.3 305.3,99.0 341.5,80.8 377.6,82.1 413.8,77.1 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,106.4 124.4,102.7 160.5,103.3 196.7,95.7 232.9,97.9 269.1,95.3 305.3,86.5 341.5,67.4 377.6,79.0 413.8,70.1 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,106.6 124.4,102.5 160.5,103.5 196.7,88.8 232.9,87.8 269.1,85.3 305.3,86.9 341.5,65.9 377.6,71.4 413.8,70.2 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.6 88.2,106.0 124.4,108.5 160.5,98.3 196.7,88.2 232.9,84.0 269.1,79.4 305.3,49.1 341.5,59.8 377.6,66.3 413.8,61.1 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.89 µs | 2.12 µs | 4.16 µs | 5.24 µs | 6.9 µs |
| D38 | 2.24 µs | 5.77 µs | 7.17 µs | 8.45 µs | 9.71 µs |
| D57 | 4.43 µs | 4.53 µs | 4.44 µs | 4.5 µs | 2.63 µs |
| D76 | 3.2 µs | 4.77 µs | 4.47 µs | 3.68 µs | 4.87 µs |
| D115 | 8.34 µs | 8.69 µs | 9.57 µs | 10.5 µs | 10.7 µs |
| D153 | 8.4 µs | 8.66 µs | 9.34 µs | 10.9 µs | 11.9 µs |
| D230 | 11.4 µs | 10.9 µs | 8.99 µs | 15.4 µs | 17.3 µs |
| D307 | 15.7 µs | 15.7 µs | 23.1 µs | 21 µs | 26.6 µs |
| D462 | 11.3 µs | 11.5 µs | 25.4 µs | 27.9 µs | 33.9 µs |
| D616 | 28.9 µs | 43.2 µs | 52 µs | 62 µs | 79.8 µs |
| D924 | 36.3 µs | 68.4 µs | 106 µs | 111 µs | 163 µs |
| D1232 | 46.6 µs | 107 µs | 155 µs | 175 µs | 249 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.3 88.2,186.6 124.4,166.9 160.5,176.3 196.7,148.6 232.9,148.4 269.1,139.4 305.3,130.2 341.5,139.9 377.6,112.6 413.8,106.0 450.0,98.8 450.0,50.3 413.8,62.5 377.6,83.2 341.5,108.0 305.3,115.0 269.1,127.4 232.9,138.3 196.7,141.5 160.5,164.2 124.4,182.0 88.2,144.2 52.0,154.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.3 88.2,186.6 124.4,166.9 160.5,176.3 196.7,148.6 232.9,148.4 269.1,139.4 305.3,130.2 341.5,139.9 377.6,112.6 413.8,106.0 450.0,98.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.2 88.2,159.3 124.4,166.2 160.5,164.8 196.7,147.4 232.9,147.5 269.1,140.9 305.3,130.4 341.5,139.4 377.6,101.0 413.8,87.7 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.7 88.2,153.0 124.4,166.8 160.5,166.7 196.7,144.6 232.9,145.3 269.1,146.4 305.3,119.1 341.5,116.3 377.6,95.6 413.8,75.1 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.1 88.2,148.2 124.4,166.5 160.5,172.3 196.7,141.9 232.9,140.8 269.1,130.9 305.3,121.8 341.5,113.6 377.6,90.5 413.8,73.6 450.0,60.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.1 88.2,144.2 124.4,182.0 160.5,164.2 196.7,141.5 232.9,138.3 269.1,127.4 305.3,115.0 341.5,108.0 377.6,83.2 413.8,62.5 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 1.54 µs | 2.82 µs | 3.42 µs | 4.33 µs |
| D38 | 5.12 ns | 3.88 µs | 4.45 µs | 4.92 µs | 5.39 µs |
| D57 | 62.5 ns | 4.61 µs | 4.55 µs | 4.75 µs | 3 µs |
| D76 | 62.5 ns | 4.5 µs | 4.73 µs | 4.17 µs | 5.63 µs |
| D115 | 146 ns | 8.25 µs | 9.57 µs | 11.3 µs | 11.7 µs |
| D153 | 199 ns | 8.62 µs | 9.81 µs | 11.8 µs | 13.1 µs |
| D230 | 342 ns | 11.4 µs | 9.5 µs | 18.4 µs | 21.5 µs |
| D307 | 366 ns | 17.4 µs | 23.1 µs | 26.2 µs | 32.3 µs |
| D462 | 536 ns | 39.8 µs | 148 µs | 208 µs | 284 µs |
| D616 | 827 ns | 191 µs | 329 µs | 353 µs | 569 µs |
| D924 | 758 ns | 418 µs | 488 µs | 763 µs | 1.74 ms |
| D1232 | 1.24 µs | 761 µs | 791 µs | 1.79 ms | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,189.7 124.4,158.7 160.5,158.7 196.7,148.2 232.9,144.3 269.1,137.6 305.3,136.8 341.5,132.0 377.6,126.6 413.8,127.7 450.0,121.6 450.0,26.4 413.8,31.7 377.6,45.6 341.5,54.2 305.3,81.2 269.1,86.2 232.9,92.4 196.7,93.7 160.5,102.8 124.4,110.7 88.2,103.4 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,189.7 124.4,158.7 160.5,158.7 196.7,148.2 232.9,144.3 269.1,137.6 305.3,136.8 341.5,132.0 377.6,126.6 413.8,127.7 450.0,121.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.9 88.2,107.5 124.4,105.3 160.5,105.6 196.7,98.1 232.9,97.6 269.1,94.1 305.3,88.8 341.5,78.6 377.6,59.1 413.8,49.4 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,105.7 124.4,105.5 160.5,105.0 196.7,96.3 232.9,95.9 269.1,96.4 305.3,85.3 341.5,62.3 377.6,52.4 413.8,47.5 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,104.5 124.4,104.9 160.5,106.6 196.7,94.2 232.9,93.6 269.1,88.1 305.3,83.8 341.5,58.1 377.6,51.5 413.8,41.9 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,103.4 124.4,110.7 160.5,102.8 196.7,93.7 232.9,92.4 269.1,86.2 305.3,81.2 341.5,54.2 377.6,45.6 413.8,31.7 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.29 ns | 10.3 ns | 15.7 ns | 28.8 ns | 28.8 ns |
| D38 | 6.39 ns | 19.5 ns | 31.3 ns | 414 ns | 605 ns |
| D57 | 177 ns | 223 ns | 491 ns | 774 ns | 414 ns |
| D76 | 162 ns | 277 ns | 770 ns | 581 ns | 1.21 µs |
| D115 | 136 ns | 649 ns | 927 ns | 1.51 µs | 1.66 µs |
| D153 | 127 ns | 1.11 µs | 1.6 µs | 1.84 µs | 2.61 µs |
| D230 | 158 ns | 1.54 µs | 1.38 µs | 3.42 µs | 4.18 µs |
| D307 | 144 ns | 1.92 µs | 3.66 µs | 4.18 µs | 7 µs |
| D462 | 172 ns | 2.11 µs | 6.02 µs | 9.34 µs | 11.5 µs |
| D616 | 244 ns | 6.38 µs | 10.4 µs | 14.1 µs | 20.6 µs |
| D924 | 171 ns | 10.4 µs | 17.3 µs | 23.3 µs | 35.5 µs |
| D1232 | 264 ns | 15.3 µs | 27.6 µs | 31.8 µs | 60 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.1 88.2,177.8 124.4,120.1 160.5,121.6 196.7,124.6 232.9,125.9 269.1,122.0 305.3,123.7 341.5,120.6 377.6,114.5 413.8,120.7 450.0,113.1 450.0,18.9 413.8,28.0 377.6,37.5 341.5,47.6 305.3,56.2 269.1,65.2 232.9,73.3 196.7,81.2 160.5,86.7 124.4,105.3 88.2,98.7 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.1 88.2,177.8 124.4,120.1 160.5,121.6 196.7,124.6 232.9,125.9 269.1,122.0 305.3,123.7 341.5,120.6 377.6,114.5 413.8,120.7 450.0,113.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.5 88.2,158.4 124.4,116.0 160.5,112.3 196.7,97.5 232.9,88.2 269.1,82.5 305.3,78.7 341.5,77.0 377.6,57.8 413.8,49.3 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.2 88.2,150.2 124.4,102.4 160.5,94.6 196.7,91.3 232.9,81.8 269.1,84.4 305.3,67.5 341.5,58.8 377.6,49.3 413.8,40.5 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,105.3 124.4,94.5 160.5,99.4 196.7,82.8 232.9,79.4 269.1,68.7 305.3,65.2 341.5,51.2 377.6,44.0 413.8,35.3 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,98.7 124.4,105.3 160.5,86.7 196.7,81.2 232.9,73.3 269.1,65.2 305.3,56.2 341.5,47.6 377.6,37.5 413.8,28.0 450.0,18.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
