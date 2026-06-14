# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.79 µs | 5.15 µs | 7.7 µs | 7.7 µs |
| D38 | 5.73 µs | 5.81 µs | 8.7 µs | 10.9 µs | 9.65 µs |
| D57 | 298 ns | 464 ns | 717 ns | 1.04 µs | 1.14 µs |
| D76 | 534 ns | 759 ns | 1.12 µs | 1.12 µs | 1.91 µs |
| D115 | 505 ns | 2.31 µs | 1.45 µs | 3.53 µs | 4.29 µs |
| D153 | 354 ns | 2.64 µs | 3.49 µs | 5.77 µs | 6.88 µs |
| D230 | 547 ns | 4.11 µs | 8.4 µs | 8.59 µs | 12.1 µs |
| D307 | 566 ns | 6.05 µs | 12 µs | 14 µs | 22.6 µs |
| D462 | 635 ns | 11.6 µs | 21.1 µs | 33.2 µs | 42 µs |
| D616 | 674 ns | 15.7 µs | 37.3 µs | 63.6 µs | 77.9 µs |
| D924 | 581 ns | 31.2 µs | 82.5 µs | 133 µs | 181 µs |
| D1232 | 568 ns | 50.9 µs | 150 µs | 250 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,122.1 124.4,186.3 160.5,173.6 196.7,174.8 232.9,182.6 269.1,173.1 305.3,172.4 341.5,169.9 377.6,168.6 413.8,171.8 450.0,172.3 450.0,33.1 413.8,47.2 377.6,65.4 341.5,78.8 305.3,92.3 269.1,105.8 232.9,118.1 196.7,128.4 160.5,146.0 124.4,157.1 88.2,110.8 52.0,115.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,122.1 124.4,186.3 160.5,173.6 196.7,174.8 232.9,182.6 269.1,173.1 305.3,172.4 341.5,169.9 377.6,168.6 413.8,171.8 450.0,172.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,121.8 124.4,176.7 160.5,166.0 196.7,141.8 232.9,138.9 269.1,129.3 305.3,120.9 341.5,106.8 377.6,100.2 413.8,85.3 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,113.0 124.4,167.2 160.5,157.6 196.7,152.0 232.9,132.8 269.1,113.8 305.3,106.1 341.5,93.8 377.6,81.4 413.8,64.2 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,108.1 124.4,159.2 160.5,157.5 196.7,132.6 232.9,122.0 269.1,113.3 305.3,102.8 341.5,83.9 377.6,69.8 413.8,53.8 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,110.8 124.4,157.1 160.5,146.0 196.7,128.4 232.9,118.1 269.1,105.8 305.3,92.3 341.5,78.8 377.6,65.4 413.8,47.2 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 4.79 µs | 6.25 µs | 6.74 µs | 7.26 µs |
| D38 | 2.11 ns | 6.77 µs | 8.23 µs | 9.5 µs | 9.85 µs |
| D57 | 2.31 ns | 3.27 µs | 4.06 µs | 5.97 µs | 8.72 µs |
| D76 | 3.2 ns | 6.04 µs | 7.31 µs | 8.27 µs | 11.8 µs |
| D115 | 17.5 ns | 6.36 µs | 7.65 µs | 18.8 µs | 19.4 µs |
| D153 | 21.7 ns | 7.36 µs | 15.6 µs | 23 µs | 33.9 µs |
| D230 | 51.4 ns | 13.4 µs | 23.2 µs | 38.7 µs | 65.8 µs |
| D307 | 84.2 ns | 14.8 µs | 37.5 µs | 65.2 µs | 122 µs |
| D462 | 127 ns | 21.7 µs | 67.8 µs | 152 µs | 217 µs |
| D616 | 141 ns | 38 µs | 130 µs | 287 µs | 409 µs |
| D924 | 217 ns | 84.5 µs | 263 µs | 602 µs | 903 µs |
| D1232 | 261 ns | 138 µs | 438 µs | 986 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,200.7 124.4,199.6 160.5,195.6 196.7,174.5 232.9,171.8 269.1,161.1 305.3,155.0 341.5,149.9 377.6,148.6 413.8,143.3 450.0,140.9 450.0,25.6 413.8,39.8 377.6,49.7 341.5,57.6 305.3,64.7 269.1,72.3 232.9,80.6 196.7,87.5 160.5,93.6 124.4,97.4 88.2,95.9 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,200.7 124.4,199.6 160.5,195.6 196.7,174.5 232.9,171.8 269.1,161.1 305.3,155.0 341.5,149.9 377.6,148.6 413.8,143.3 450.0,140.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,100.5 124.4,109.6 160.5,102.0 196.7,101.3 232.9,99.5 269.1,92.1 305.3,90.8 341.5,86.1 377.6,79.2 413.8,69.2 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,98.1 124.4,106.9 160.5,99.6 196.7,99.0 232.9,90.2 269.1,85.3 305.3,79.3 341.5,72.0 377.6,63.8 413.8,55.1 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.6 88.2,96.4 124.4,102.1 160.5,98.1 196.7,87.9 232.9,85.4 269.1,78.9 305.3,72.4 341.5,61.9 377.6,54.1 413.8,44.9 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,95.9 124.4,97.4 160.5,93.6 196.7,87.5 232.9,80.6 269.1,72.3 305.3,64.7 341.5,57.6 377.6,49.7 413.8,39.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 39.7 ns | 42.4 ns | 40.2 ns | 39.6 ns |
| D38 | 15.4 ns | 42.8 ns | 74.4 ns | 75.2 ns | 94.7 ns |
| D57 | 16.2 ns | 37.3 ns | 67.3 ns | 622 ns | 704 ns |
| D76 | 17.3 ns | 73.1 ns | 603 ns | 485 ns | 898 ns |
| D115 | 21.9 ns | 72.6 ns | 322 ns | 1.13 µs | 1.01 µs |
| D153 | 23.2 ns | 609 ns | 923 ns | 1.27 µs | 2.12 µs |
| D230 | 29.2 ns | 638 ns | 1.42 µs | 2.06 µs | 2.49 µs |
| D307 | 42.2 ns | 1.09 µs | 2.1 µs | 2.52 µs | 5.61 µs |
| D462 | 62.4 ns | 1.52 µs | 2.8 µs | 6.27 µs | 8.46 µs |
| D616 | 68 ns | 2.36 µs | 5.99 µs | 11.2 µs | 15.9 µs |
| D924 | 116 ns | 3.67 µs | 11.1 µs | 24.8 µs | 28.5 µs |
| D1232 | 86.2 ns | 6.24 µs | 20.7 µs | 27.1 µs | 50.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,200.6 124.4,199.6 160.5,198.1 196.7,192.9 232.9,191.7 269.1,186.7 305.3,178.7 341.5,170.2 377.6,168.4 413.8,156.8 450.0,163.2 450.0,24.7 413.8,37.3 377.6,49.9 341.5,63.6 305.3,72.6 269.1,90.2 232.9,93.7 196.7,109.7 160.5,112.3 124.4,117.6 88.2,161.2 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,200.6 124.4,199.6 160.5,198.1 196.7,192.9 232.9,191.7 269.1,186.7 305.3,178.7 341.5,170.2 377.6,168.4 413.8,156.8 450.0,163.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,178.4 124.4,181.4 160.5,166.8 196.7,166.9 232.9,120.8 269.1,119.7 305.3,108.2 341.5,100.9 377.6,91.4 413.8,81.7 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.7 88.2,166.4 124.4,168.6 160.5,121.0 196.7,134.6 232.9,111.7 269.1,102.3 305.3,93.9 341.5,87.6 377.6,71.1 413.8,57.8 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,166.2 124.4,120.3 160.5,125.7 196.7,107.4 232.9,104.8 269.1,94.3 305.3,89.9 341.5,70.1 377.6,57.4 413.8,40.2 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,161.2 124.4,117.6 160.5,112.3 196.7,109.7 232.9,93.7 269.1,90.2 305.3,72.6 341.5,63.6 377.6,49.9 413.8,37.3 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 301 ns | 374 ns | 369 ns | 377 ns |
| D38 | 2.11 ns | 394 ns | 402 ns | 400 ns | 372 ns |
| D57 | 254 ns | 408 ns | 436 ns | 480 ns | 557 ns |
| D76 | 276 ns | 493 ns | 480 ns | 497 ns | 650 ns |
| D115 | 275 ns | 453 ns | 303 ns | 1.05 µs | 958 ns |
| D153 | 322 ns | 537 ns | 688 ns | 1.08 µs | 1.24 µs |
| D230 | 476 ns | 723 ns | 1.12 µs | 1.11 µs | 1.57 µs |
| D307 | 642 ns | 650 ns | 1.2 µs | 1.23 µs | 10 µs |
| D462 | 1.13 µs | 2.75 µs | 3.05 µs | 3.76 µs | 4.27 µs |
| D616 | 1.33 µs | 1.56 µs | 1.75 µs | 2.81 µs | 3.39 µs |
| D924 | 1.99 µs | 1.96 µs | 2.73 µs | 3.55 µs | 4.29 µs |
| D1232 | 2.31 µs | 3.04 µs | 3.98 µs | 4.82 µs | 6.56 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,197.0 124.4,113.8 160.5,112.3 196.7,112.4 232.9,109.7 269.1,102.9 305.3,97.7 341.5,87.8 377.6,85.1 413.8,78.1 450.0,75.5 450.0,57.3 413.8,64.7 377.6,68.8 341.5,64.8 305.3,50.0 269.1,82.1 232.9,86.3 196.7,90.7 160.5,97.5 124.4,100.2 88.2,107.2 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,197.0 124.4,113.8 160.5,112.3 196.7,112.4 232.9,109.7 269.1,102.9 305.3,97.7 341.5,87.8 377.6,85.1 413.8,78.1 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.9 88.2,106.2 124.4,105.6 160.5,102.3 196.7,103.8 232.9,100.8 269.1,95.6 305.3,97.5 341.5,72.4 377.6,82.2 413.8,78.3 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,105.8 124.4,104.4 160.5,102.8 196.7,110.8 232.9,96.5 269.1,88.0 305.3,86.9 341.5,70.7 377.6,80.3 413.8,72.6 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,105.9 124.4,102.7 160.5,102.2 196.7,89.2 232.9,88.6 269.1,88.1 305.3,86.4 341.5,67.0 377.6,72.1 413.8,68.0 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,107.2 124.4,100.2 160.5,97.5 196.7,90.7 232.9,86.3 269.1,82.1 305.3,50.0 341.5,64.8 377.6,68.8 413.8,64.7 450.0,57.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.15 µs | 9.23 µs | 11.6 µs | 12.7 µs | 13.9 µs |
| D38 | 8.14 µs | 12.5 µs | 15.5 µs | 18.3 µs | 19.7 µs |
| D57 | 3.58 µs | 3.62 µs | 4.38 µs | 4.93 µs | 4.68 µs |
| D76 | 4.44 µs | 4.86 µs | 4.91 µs | 4.01 µs | 5.34 µs |
| D115 | 7.53 µs | 8.31 µs | 5.18 µs | 10.2 µs | 8.84 µs |
| D153 | 8.19 µs | 9.13 µs | 9.81 µs | 10.7 µs | 10.6 µs |
| D230 | 10.4 µs | 12.8 µs | 14.7 µs | 12.8 µs | 14.5 µs |
| D307 | 15.8 µs | 18 µs | 22.8 µs | 19.8 µs | 28.1 µs |
| D462 | 15.9 µs | 20.3 µs | 19.9 µs | 27.6 µs | 28.2 µs |
| D616 | 27.3 µs | 42.7 µs | 50.9 µs | 67.8 µs | 71.8 µs |
| D924 | 42.9 µs | 72.6 µs | 95.5 µs | 134 µs | 149 µs |
| D1232 | 47.2 µs | 112 µs | 166 µs | 218 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,149.3 124.4,173.1 160.5,166.8 196.7,151.5 232.9,149.1 269.1,142.2 305.3,130.0 341.5,130.0 377.6,114.3 413.8,101.2 450.0,98.4 450.0,47.9 413.8,65.1 377.6,86.3 341.5,113.4 305.3,113.4 269.1,132.5 232.9,141.7 196.7,146.9 160.5,161.5 124.4,165.3 88.2,123.6 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,149.3 124.4,173.1 160.5,166.8 196.7,151.5 232.9,149.1 269.1,142.2 305.3,130.0 341.5,130.0 377.6,114.3 413.8,101.2 450.0,98.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.7 88.2,137.0 124.4,172.7 160.5,164.2 196.7,148.7 232.9,146.0 269.1,136.2 305.3,126.3 341.5,122.8 377.6,101.3 413.8,85.9 450.0,73.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,130.6 124.4,167.3 160.5,163.9 196.7,162.4 232.9,143.9 269.1,132.2 305.3,119.5 341.5,123.4 377.6,96.2 413.8,78.0 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.4 88.2,125.8 124.4,163.8 160.5,169.8 196.7,142.8 232.9,141.4 269.1,136.3 305.3,123.5 341.5,114.0 377.6,87.9 413.8,68.2 450.0,54.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,123.6 124.4,165.3 160.5,161.5 196.7,146.9 232.9,141.7 269.1,132.5 305.3,113.4 341.5,113.4 377.6,86.3 413.8,65.1 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 5.29 µs | 6.87 µs | 7.38 µs | 7.94 µs |
| D38 | 6.48 ns | 7.46 µs | 8.95 µs | 10.2 µs | 10.6 µs |
| D57 | 54 ns | 3.76 µs | 4.31 µs | 4.94 µs | 4.96 µs |
| D76 | 79.6 ns | 4.33 µs | 4.87 µs | 4.52 µs | 5.85 µs |
| D115 | 137 ns | 8.14 µs | 5.33 µs | 10.8 µs | 9.49 µs |
| D153 | 191 ns | 9.05 µs | 10.2 µs | 11.7 µs | 11.9 µs |
| D230 | 284 ns | 13.3 µs | 16 µs | 16.1 µs | 17.8 µs |
| D307 | 367 ns | 18.8 µs | 22.7 µs | 23.3 µs | 34.4 µs |
| D462 | 653 ns | 69.5 µs | 116 µs | 204 µs | 250 µs |
| D616 | 747 ns | 193 µs | 328 µs | 378 µs | 530 µs |
| D924 | 983 ns | 450 µs | 454 µs | 848 µs | 1.62 ms |
| D1232 | 1.09 µs | 834 µs | 853 µs | 2.3 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.8 124.4,160.5 160.5,155.7 196.7,149.0 232.9,144.8 269.1,139.9 305.3,136.7 341.5,129.6 377.6,127.9 413.8,124.5 450.0,123.2 450.0,25.0 413.8,32.6 377.6,46.5 341.5,55.8 305.3,80.4 269.1,88.6 232.9,93.6 196.7,96.4 160.5,102.4 124.4,104.4 88.2,95.0 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.8 124.4,160.5 160.5,155.7 196.7,149.0 232.9,144.8 269.1,139.9 305.3,136.7 341.5,129.6 377.6,127.9 413.8,124.5 450.0,123.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,99.3 124.4,107.8 160.5,106.1 196.7,98.3 232.9,97.0 269.1,92.2 305.3,87.9 341.5,71.6 377.6,59.0 413.8,48.5 450.0,40.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.4 88.2,97.1 124.4,106.2 160.5,104.7 196.7,103.5 232.9,95.5 269.1,89.9 305.3,85.6 341.5,65.3 377.6,52.4 413.8,48.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.5 88.2,95.4 124.4,104.5 160.5,105.6 196.7,94.7 232.9,93.8 269.1,89.8 305.3,85.2 341.5,58.3 377.6,50.6 413.8,40.6 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,95.0 124.4,104.4 160.5,102.4 196.7,96.4 232.9,93.6 269.1,88.6 305.3,80.4 341.5,55.8 377.6,46.5 413.8,32.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.92 ns | 14.5 ns | 19.3 ns | 29.9 ns | 28.8 ns |
| D38 | 9.01 ns | 36.5 ns | 32.3 ns | 1.64 µs | 2.85 µs |
| D57 | 139 ns | 187 ns | 486 ns | 676 ns | 764 ns |
| D76 | 210 ns | 277 ns | 647 ns | 628 ns | 1.05 µs |
| D115 | 121 ns | 661 ns | 502 ns | 1.52 µs | 1.28 µs |
| D153 | 126 ns | 1.07 µs | 1.52 µs | 1.85 µs | 2.68 µs |
| D230 | 145 ns | 1.71 µs | 2.37 µs | 3.02 µs | 3.38 µs |
| D307 | 146 ns | 2.3 µs | 3.71 µs | 3.88 µs | 7.15 µs |
| D462 | 183 ns | 3.67 µs | 5.07 µs | 9.38 µs | 10.1 µs |
| D616 | 225 ns | 6.4 µs | 10.6 µs | 14.1 µs | 20.5 µs |
| D924 | 231 ns | 11.4 µs | 17.2 µs | 26.7 µs | 34.7 µs |
| D1232 | 205 ns | 16.3 µs | 28.2 µs | 40.6 µs | 61.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.4 88.2,171.8 124.4,124.3 160.5,117.1 196.7,126.6 232.9,126.0 269.1,123.6 305.3,123.4 341.5,119.5 377.6,115.9 413.8,115.5 450.0,117.6 450.0,18.3 413.8,28.4 377.6,37.5 341.5,49.7 305.3,55.8 269.1,68.9 232.9,72.9 196.7,85.8 160.5,89.2 124.4,94.7 88.2,71.8 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.4 88.2,171.8 124.4,124.3 160.5,117.1 196.7,126.6 232.9,126.0 269.1,123.6 305.3,123.4 341.5,119.5 377.6,115.9 413.8,115.5 450.0,117.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.5 88.2,147.5 124.4,119.2 160.5,112.3 196.7,97.2 232.9,88.8 269.1,80.7 305.3,75.5 341.5,67.4 377.6,57.8 413.8,47.7 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,149.6 124.4,102.5 160.5,97.6 196.7,102.0 232.9,82.7 269.1,75.0 305.3,67.2 341.5,61.8 377.6,49.0 413.8,40.6 450.0,32.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.0 88.2,81.4 124.4,96.8 160.5,98.1 196.7,82.7 232.9,79.3 269.1,70.8 305.3,66.5 341.5,51.1 377.6,44.1 413.8,32.9 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,71.8 124.4,94.7 160.5,89.2 196.7,85.8 232.9,72.9 269.1,68.9 305.3,55.8 341.5,49.7 377.6,37.5 413.8,28.4 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
