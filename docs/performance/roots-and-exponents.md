# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 76.9 ns | 104 ns | 155 ns | 258 ns | 206 ns |
| D38 | 76.6 ns | 122 ns | 264 ns | 373 ns | 396 ns |
| D57 | 191 ns | 343 ns | 701 ns | 808 ns | 1.13 µs |
| D76 | 181 ns | 417 ns | 818 ns | 1.3 µs | 1.67 µs |
| D115 | 226 ns | 766 ns | 1.35 µs | 2.03 µs | 2.99 µs |
| D153 | 250 ns | 852 ns | 1.15 µs | 2.45 µs | 4.33 µs |
| D230 | 293 ns | 1.41 µs | 3.07 µs | 5.17 µs | 8.76 µs |
| D307 | 302 ns | 1.79 µs | 4.67 µs | 8.89 µs | 13.7 µs |
| D462 | 345 ns | 2.24 µs | 8.85 µs | 17 µs | 30.2 µs |
| D616 | 407 ns | 4.48 µs | 14.3 µs | 34 µs | 47.6 µs |
| D924 | 590 ns | 9 µs | 20.6 µs | 62.6 µs | 130 µs |
| D1232 | 510 ns | 13.3 µs | 52.3 µs | 120 µs | 174 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.6 88.2,174.6 124.4,158.8 160.5,159.7 196.7,155.8 232.9,154.1 269.1,151.3 305.3,150.8 341.5,148.5 377.6,145.6 413.8,139.2 450.0,141.7 450.0,40.4 413.8,45.5 377.6,62.9 341.5,70.8 305.3,84.6 269.1,92.3 232.9,104.5 196.7,111.0 160.5,121.1 124.4,127.8 88.2,146.1 52.0,157.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.6 88.2,174.6 124.4,158.8 160.5,159.7 196.7,155.8 232.9,154.1 269.1,151.3 305.3,150.8 341.5,148.5 377.6,145.6 413.8,139.2 450.0,141.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.4 88.2,166.5 124.4,148.6 160.5,145.2 196.7,134.6 232.9,132.8 269.1,124.0 305.3,119.9 341.5,116.0 377.6,104.0 413.8,91.8 450.0,85.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.3 88.2,153.1 124.4,136.2 160.5,133.5 196.7,124.7 232.9,127.6 269.1,110.5 305.3,103.2 341.5,92.1 377.6,83.7 413.8,77.4 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,147.1 124.4,133.7 160.5,125.5 196.7,117.7 232.9,114.4 269.1,101.5 305.3,92.0 341.5,80.8 377.6,68.7 413.8,58.1 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.5 88.2,146.1 124.4,127.8 160.5,121.1 196.7,111.0 232.9,104.5 269.1,92.3 305.3,84.6 341.5,70.8 377.6,62.9 413.8,45.5 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.44 µs | 3.21 µs | 3.43 µs | 3.18 µs |
| D38 | 1.56 ns | 2.79 µs | 4.17 µs | 4.26 µs | 4.72 µs |
| D57 | 2.81 ns | 2.62 µs | 4.42 µs | 4.71 µs | 9.17 µs |
| D76 | 2.39 ns | 5.04 µs | 7.25 µs | 9.4 µs | 11.5 µs |
| D115 | 11.9 ns | 7.14 µs | 13.9 µs | 18.1 µs | 22.2 µs |
| D153 | 13.3 ns | 7.6 µs | 9.8 µs | 16.6 µs | 34.3 µs |
| D230 | 46 ns | 12.2 µs | 22.2 µs | 42.4 µs | 78.5 µs |
| D307 | 68.3 ns | 15.9 µs | 38.3 µs | 79.7 µs | 117 µs |
| D462 | 87.3 ns | 15 µs | 81.1 µs | 153 µs | 230 µs |
| D616 | 96.3 ns | 35.2 µs | 133 µs | 287 µs | 401 µs |
| D924 | 205 ns | 81.2 µs | 178 µs | 565 µs | 985 µs |
| D1232 | 184 ns | 111 µs | 409 µs | 914 µs | 2.18 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,204.5 124.4,197.2 160.5,199.2 196.7,179.3 232.9,177.9 269.1,162.5 305.3,157.6 341.5,154.5 377.6,153.3 413.8,143.9 450.0,145.3 450.0,28.9 413.8,38.8 377.6,49.9 341.5,56.8 305.3,65.2 269.1,70.1 232.9,80.4 196.7,85.8 160.5,94.0 124.4,96.8 88.2,105.0 52.0,109.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,204.5 124.4,197.2 160.5,199.2 196.7,179.3 232.9,177.9 269.1,162.5 305.3,157.6 341.5,154.5 377.6,153.3 413.8,143.9 450.0,145.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.8 88.2,111.6 124.4,112.4 160.5,104.2 196.7,99.9 232.9,99.1 269.1,93.3 305.3,90.0 341.5,90.7 377.6,80.1 413.8,69.7 450.0,65.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.6 124.4,105.9 160.5,99.7 196.7,91.6 232.9,96.0 269.1,85.8 305.3,79.0 341.5,69.7 377.6,63.6 413.8,60.0 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,106.3 124.4,105.1 160.5,96.5 196.7,88.4 232.9,89.4 269.1,77.8 305.3,70.0 341.5,61.8 377.6,54.1 413.8,45.7 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,105.0 124.4,96.8 160.5,94.0 196.7,85.8 232.9,80.4 269.1,70.1 305.3,65.2 341.5,56.8 377.6,49.9 413.8,38.8 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.7 ns | 30.8 ns | 42.6 ns | 40.8 ns | 31.1 ns |
| D38 | 15.5 ns | 33.2 ns | 74.3 ns | 66.9 ns | 95 ns |
| D57 | 16.5 ns | 30 ns | 83.1 ns | 329 ns | 390 ns |
| D76 | 12.6 ns | 75.2 ns | 371 ns | 381 ns | 621 ns |
| D115 | 22.4 ns | 96.4 ns | 420 ns | 816 ns | 958 ns |
| D153 | 15.8 ns | 420 ns | 430 ns | 742 ns | 1.6 µs |
| D230 | 28.8 ns | 410 ns | 1.08 µs | 1.88 µs | 2.84 µs |
| D307 | 43.5 ns | 683 ns | 1.74 µs | 2.82 µs | 4.91 µs |
| D462 | 47.9 ns | 765 ns | 3.13 µs | 5.73 µs | 7.2 µs |
| D616 | 51 ns | 1.95 µs | 5.66 µs | 10.9 µs | 13.7 µs |
| D924 | 103 ns | 3.24 µs | 6.85 µs | 22.1 µs | 26.9 µs |
| D1232 | 52.5 ns | 5.23 µs | 18.7 µs | 23.8 µs | 38.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,200.5 124.4,199.2 160.5,205.0 196.7,192.5 232.9,200.0 269.1,187.0 305.3,178.1 341.5,176.0 377.6,174.6 413.8,159.4 450.0,174.0 450.0,30.9 413.8,38.5 377.6,53.2 341.5,67.1 305.3,75.4 269.1,87.3 232.9,99.7 196.7,110.9 160.5,120.3 124.4,130.4 88.2,161.1 52.0,185.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,200.5 124.4,199.2 160.5,205.0 196.7,192.5 232.9,200.0 269.1,187.0 305.3,178.1 341.5,176.0 377.6,174.6 413.8,159.4 450.0,174.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,185.6 88.2,183.9 124.4,186.1 160.5,166.2 196.7,160.8 232.9,128.8 269.1,129.4 305.3,118.3 341.5,115.8 377.6,95.5 413.8,84.4 450.0,74.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.5 88.2,166.4 124.4,164.0 160.5,131.5 196.7,128.8 232.9,128.3 269.1,108.3 305.3,97.9 341.5,85.2 377.6,72.4 413.8,68.2 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,168.7 124.4,134.1 160.5,131.0 196.7,114.4 232.9,116.5 269.1,96.3 305.3,87.5 341.5,72.1 377.6,58.1 413.8,42.8 450.0,41.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.4 88.2,161.1 124.4,130.4 160.5,120.3 196.7,110.9 232.9,99.7 269.1,87.3 305.3,75.4 341.5,67.1 377.6,53.2 413.8,38.5 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 234 ns | 377 ns | 380 ns | 334 ns |
| D38 | 2.49 ns | 326 ns | 410 ns | 387 ns | 387 ns |
| D57 | 278 ns | 342 ns | 482 ns | 399 ns | 609 ns |
| D76 | 209 ns | 415 ns | 493 ns | 623 ns | 635 ns |
| D115 | 291 ns | 537 ns | 722 ns | 943 ns | 1.02 µs |
| D153 | 231 ns | 558 ns | 371 ns | 818 ns | 1.3 µs |
| D230 | 564 ns | 577 ns | 1.03 µs | 1.28 µs | 1.81 µs |
| D307 | 661 ns | 713 ns | 1.2 µs | 1.36 µs | 10.5 µs |
| D462 | 1.04 µs | 1.97 µs | 3.3 µs | 3.92 µs | 5.01 µs |
| D616 | 1.22 µs | 1.41 µs | 1.9 µs | 3.04 µs | 3.06 µs |
| D924 | 2.12 µs | 2.04 µs | 1.92 µs | 3.59 µs | 5.25 µs |
| D1232 | 1.67 µs | 2.3 µs | 4.21 µs | 5.07 µs | 5.87 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,194.1 124.4,112.2 160.5,117.2 196.7,111.5 232.9,115.5 269.1,100.0 305.3,97.2 341.5,89.4 377.6,86.5 413.8,76.9 450.0,81.1 450.0,59.3 413.8,61.2 377.6,70.6 341.5,62.0 305.3,49.2 269.1,79.7 232.9,85.4 196.7,89.7 160.5,97.9 124.4,98.6 88.2,106.5 52.0,109.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,194.1 124.4,112.2 160.5,117.2 196.7,111.5 232.9,115.5 269.1,100.0 305.3,97.2 341.5,89.4 377.6,86.5 413.8,76.9 450.0,81.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.2 88.2,109.5 124.4,108.6 160.5,105.3 196.7,100.8 232.9,100.1 269.1,99.6 305.3,95.9 341.5,78.2 377.6,84.1 413.8,77.6 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,105.5 124.4,102.7 160.5,102.3 196.7,95.7 232.9,107.2 269.1,89.5 305.3,86.8 341.5,69.3 377.6,78.8 413.8,78.7 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,106.5 124.4,106.0 160.5,98.2 196.7,91.0 232.9,93.5 269.1,85.7 305.3,84.7 341.5,66.3 377.6,70.7 413.8,67.8 450.0,61.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.1 88.2,106.5 124.4,98.6 160.5,97.9 196.7,89.7 232.9,85.4 269.1,79.7 305.3,49.2 341.5,62.0 377.6,70.6 413.8,61.2 450.0,59.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.74 µs | 2.66 µs | 5.79 µs | 6.3 µs | 5.88 µs |
| D38 | 2.74 µs | 4.85 µs | 7.62 µs | 8.63 µs | 9.98 µs |
| D57 | 2.75 µs | 1.66 µs | 2.66 µs | 2.1 µs | 2.9 µs |
| D76 | 1.66 µs | 2.32 µs | 2.76 µs | 2.93 µs | 2.98 µs |
| D115 | 4.43 µs | 5.31 µs | 5.67 µs | 5.56 µs | 5.88 µs |
| D153 | 3.5 µs | 5.35 µs | 3.41 µs | 4.48 µs | 6.26 µs |
| D230 | 6.54 µs | 6.25 µs | 8.21 µs | 8.8 µs | 9.93 µs |
| D307 | 9.62 µs | 11.1 µs | 13.6 µs | 14.3 µs | 15.7 µs |
| D462 | 7.56 µs | 7.89 µs | 14.5 µs | 16.7 µs | 18.6 µs |
| D616 | 12.7 µs | 23.3 µs | 29.7 µs | 38.7 µs | 37.6 µs |
| D924 | 24.8 µs | 39.5 µs | 36.9 µs | 70.6 µs | 92.7 µs |
| D1232 | 17.2 µs | 55 µs | 89.4 µs | 115 µs | 123 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.8 88.2,180.8 124.4,180.7 160.5,195.3 196.7,166.9 232.9,173.8 269.1,155.6 305.3,144.4 341.5,151.4 377.6,136.5 413.8,117.1 450.0,127.6 450.0,70.6 413.8,78.9 377.6,105.0 341.5,125.4 305.3,130.3 269.1,143.5 232.9,156.9 196.7,158.7 160.5,178.4 124.4,179.2 88.2,143.4 52.0,158.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.8 88.2,180.8 124.4,180.7 160.5,195.3 196.7,166.9 232.9,173.8 269.1,155.6 305.3,144.4 341.5,151.4 377.6,136.5 413.8,117.1 450.0,127.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.7 88.2,164.3 124.4,195.3 160.5,185.6 196.7,161.6 232.9,161.4 269.1,156.9 305.3,140.4 341.5,150.2 377.6,118.9 413.8,103.6 450.0,94.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.1 88.2,151.2 124.4,181.7 160.5,180.6 196.7,159.8 232.9,174.5 269.1,149.0 305.3,134.3 341.5,132.6 377.6,111.8 413.8,105.5 450.0,79.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.7 88.2,147.6 124.4,188.5 160.5,178.9 196.7,160.3 232.9,166.6 269.1,147.0 305.3,133.0 341.5,128.5 377.6,104.2 413.8,86.7 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,143.4 124.4,179.2 160.5,178.4 196.7,158.7 232.9,156.9 269.1,143.5 305.3,130.3 341.5,125.4 377.6,105.0 413.8,78.9 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.2 ns | 1.81 µs | 3.85 µs | 4.04 µs | 3.74 µs |
| D38 | 13.4 ns | 3.35 µs | 4.84 µs | 4.89 µs | 5.37 µs |
| D57 | 66.1 ns | 2.97 µs | 4.62 µs | 3.76 µs | 5.08 µs |
| D76 | 64.1 ns | 3.52 µs | 4.76 µs | 5.22 µs | 5.58 µs |
| D115 | 136 ns | 8.59 µs | 9.48 µs | 10.3 µs | 10.9 µs |
| D153 | 132 ns | 8.98 µs | 6.15 µs | 8.2 µs | 11.9 µs |
| D230 | 373 ns | 11.5 µs | 15.1 µs | 18.1 µs | 21.2 µs |
| D307 | 383 ns | 19.2 µs | 22.9 µs | 28.4 µs | 32.4 µs |
| D462 | 520 ns | 48.3 µs | 137 µs | 205 µs | 262 µs |
| D616 | 726 ns | 177 µs | 329 µs | 379 µs | 516 µs |
| D924 | 1.05 µs | 416 µs | 322 µs | 794 µs | 1.74 ms |
| D1232 | 879 ns | 692 µs | 794 µs | 2.12 ms | 2.31 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.0 88.2,205.8 124.4,182.7 160.5,183.1 196.7,172.2 232.9,172.7 269.1,157.6 305.3,157.2 341.5,152.8 377.6,148.0 413.8,142.6 450.0,145.2 450.0,31.2 413.8,35.3 377.6,52.9 341.5,62.7 305.3,93.0 269.1,99.1 232.9,107.4 196.7,108.7 160.5,118.5 124.4,119.8 88.2,119.0 52.0,124.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.0 88.2,205.8 124.4,182.7 160.5,183.1 196.7,172.2 232.9,172.7 269.1,157.6 305.3,157.2 341.5,152.8 377.6,148.0 413.8,142.6 450.0,145.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,134.7 88.2,125.8 124.4,127.6 160.5,125.1 196.7,112.2 232.9,111.6 269.1,108.0 305.3,100.6 341.5,87.2 377.6,68.4 413.8,56.0 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.8 88.2,120.5 124.4,121.2 160.5,120.7 196.7,110.8 232.9,117.0 269.1,104.1 305.3,98.0 341.5,72.1 377.6,59.4 413.8,59.7 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.1 88.2,120.4 124.4,124.2 160.5,119.4 196.7,109.6 232.9,112.9 269.1,101.4 305.3,94.9 341.5,66.3 377.6,57.4 413.8,46.7 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.2 88.2,119.0 124.4,119.8 160.5,118.5 196.7,108.7 232.9,107.4 269.1,99.1 305.3,93.0 341.5,62.7 377.6,52.9 413.8,35.3 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.91 ns | 10.4 ns | 18.4 ns | 29.5 ns | 22.8 ns |
| D38 | 7.06 ns | 16.5 ns | 44.3 ns | 135 ns | 192 ns |
| D57 | 81.7 ns | 113 ns | 207 ns | 313 ns | 372 ns |
| D76 | 76.2 ns | 117 ns | 293 ns | 384 ns | 628 ns |
| D115 | 98.7 ns | 240 ns | 411 ns | 733 ns | 921 ns |
| D153 | 92.1 ns | 342 ns | 410 ns | 713 ns | 1.56 µs |
| D230 | 145 ns | 435 ns | 959 ns | 1.84 µs | 2.74 µs |
| D307 | 154 ns | 671 ns | 1.7 µs | 2.74 µs | 4.85 µs |
| D462 | 182 ns | 706 ns | 2.63 µs | 5.87 µs | 6.95 µs |
| D616 | 209 ns | 1.84 µs | 4.65 µs | 7.4 µs | 13.2 µs |
| D924 | 272 ns | 2.68 µs | 4.8 µs | 14.9 µs | 26.2 µs |
| D1232 | 204 ns | 4.38 µs | 13.2 µs | 24.4 µs | 37.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.1 88.2,176.0 124.4,133.5 160.5,134.7 196.7,130.2 232.9,131.4 269.1,123.5 305.3,122.5 341.5,119.6 377.6,117.2 413.8,112.6 450.0,117.7 450.0,27.0 413.8,33.2 377.6,45.2 341.5,56.3 305.3,62.6 269.1,72.5 232.9,82.3 196.7,91.4 160.5,98.1 124.4,107.2 88.2,118.7 52.0,155.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.1 88.2,176.0 124.4,133.5 160.5,134.7 196.7,130.2 232.9,131.4 269.1,123.5 305.3,122.5 341.5,119.6 377.6,117.2 413.8,112.6 450.0,117.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.3 88.2,161.3 124.4,127.9 160.5,127.2 196.7,114.8 232.9,108.7 269.1,104.5 305.3,96.9 341.5,96.1 377.6,79.4 413.8,72.8 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,144.1 124.4,117.4 160.5,111.3 196.7,105.5 232.9,105.5 269.1,90.7 305.3,80.8 341.5,73.2 377.6,63.3 413.8,62.8 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.2 88.2,124.8 124.4,110.2 160.5,106.6 196.7,95.4 232.9,95.9 269.1,79.4 305.3,72.5 341.5,59.2 377.6,55.2 413.8,43.1 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.7 88.2,118.7 124.4,107.2 160.5,98.1 196.7,91.4 232.9,82.3 269.1,72.5 305.3,62.6 341.5,56.3 377.6,45.2 413.8,33.2 450.0,27.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
