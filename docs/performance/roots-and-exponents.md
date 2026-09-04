# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 76.9 ns | 131 ns | 160 ns | 234 ns | 255 ns |
| D38 | 81.3 ns | 159 ns | 263 ns | 379 ns | 410 ns |
| D57 | 198 ns | 407 ns | 695 ns | 1.13 µs | 826 ns |
| D76 | 193 ns | 486 ns | 825 ns | 1.05 µs | 1.64 µs |
| D115 | 228 ns | 792 ns | 1.35 µs | 2.03 µs | 3.66 µs |
| D153 | 241 ns | 868 ns | 1.79 µs | 3.07 µs | 4.3 µs |
| D230 | 288 ns | 1.38 µs | 2.11 µs | 3.38 µs | 8.77 µs |
| D307 | 301 ns | 1.85 µs | 4.7 µs | 8.87 µs | 10.4 µs |
| D462 | 376 ns | 2.53 µs | 9.43 µs | 16.4 µs | 33.4 µs |
| D616 | 457 ns | 4.01 µs | 14.4 µs | 23.2 µs | 50.3 µs |
| D924 | 554 ns | 9.01 µs | 30.7 µs | 62.5 µs | 84.1 µs |
| D1232 | 933 ns | 15.4 µs | 57.4 µs | 132 µs | 166 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.6 88.2,173.6 124.4,158.1 160.5,158.6 196.7,155.7 232.9,154.7 269.1,151.6 305.3,150.8 341.5,147.0 377.6,143.6 413.8,140.3 450.0,131.2 450.0,41.1 413.8,53.0 377.6,61.9 341.5,69.0 305.3,89.3 269.1,92.3 232.9,104.7 196.7,107.4 160.5,121.4 124.4,133.3 88.2,145.5 52.0,153.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.6 88.2,173.6 124.4,158.1 160.5,158.6 196.7,155.7 232.9,154.7 269.1,151.6 305.3,150.8 341.5,147.0 377.6,143.6 413.8,140.3 450.0,131.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.4 88.2,162.0 124.4,145.6 160.5,142.6 196.7,134.1 232.9,132.4 269.1,124.4 305.3,119.3 341.5,113.8 377.6,105.9 413.8,91.8 450.0,82.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,153.2 124.4,136.3 160.5,133.4 196.7,124.8 232.9,119.9 269.1,117.0 305.3,103.1 341.5,91.0 377.6,83.7 413.8,70.5 450.0,59.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,146.9 124.4,127.9 160.5,129.2 196.7,117.7 232.9,110.5 269.1,108.9 305.3,92.1 341.5,81.4 377.6,75.4 413.8,58.2 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,145.5 124.4,133.3 160.5,121.4 196.7,107.4 232.9,104.7 269.1,92.3 305.3,89.3 341.5,69.0 377.6,61.9 413.8,53.0 450.0,41.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.63 µs | 3.5 µs | 2.94 µs | 3.62 µs |
| D38 | 1.59 ns | 3.49 µs | 4.16 µs | 4.67 µs | 5.17 µs |
| D57 | 2.58 ns | 3.57 µs | 4.48 µs | 5.74 µs | 7.65 µs |
| D76 | 2.9 ns | 6.48 µs | 7.29 µs | 7.89 µs | 11.4 µs |
| D115 | 13.2 ns | 6.64 µs | 13 µs | 17.9 µs | 27.6 µs |
| D153 | 16.2 ns | 7.22 µs | 15.3 µs | 21.8 µs | 34.7 µs |
| D230 | 45.9 ns | 13.2 µs | 14.8 µs | 25.8 µs | 78.7 µs |
| D307 | 68.2 ns | 16.1 µs | 38.1 µs | 79.8 µs | 84.2 µs |
| D462 | 133 ns | 17.5 µs | 85.7 µs | 148 µs | 246 µs |
| D616 | 135 ns | 30 µs | 132 µs | 197 µs | 413 µs |
| D924 | 179 ns | 80.7 µs | 268 µs | 559 µs | 701 µs |
| D1232 | 368 ns | 132 µs | 444 µs | 990 µs | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,204.3 124.4,198.2 160.5,196.8 196.7,177.9 232.9,175.4 269.1,162.5 305.3,157.6 341.5,149.3 377.6,149.1 413.8,145.6 450.0,136.7 450.0,27.2 413.8,43.0 377.6,49.6 341.5,56.0 305.3,69.3 269.1,70.1 232.9,80.3 196.7,83.1 160.5,94.1 124.4,99.0 88.2,103.9 52.0,108.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,204.3 124.4,198.2 160.5,196.8 196.7,177.9 232.9,175.4 269.1,162.5 305.3,157.6 341.5,149.3 377.6,149.1 413.8,145.6 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,108.8 124.4,108.5 160.5,101.1 196.7,100.8 232.9,99.7 269.1,92.3 305.3,89.8 341.5,88.8 377.6,82.1 413.8,69.8 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.6 124.4,105.7 160.5,99.6 196.7,92.4 232.9,90.5 269.1,90.9 305.3,79.1 341.5,69.1 377.6,63.7 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,105.2 124.4,102.6 160.5,98.7 196.7,88.5 232.9,86.0 269.1,83.9 305.3,69.9 341.5,62.2 377.6,58.8 413.8,45.8 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,103.9 124.4,99.0 160.5,94.1 196.7,83.1 232.9,80.3 269.1,70.1 305.3,69.3 341.5,56.0 377.6,49.6 413.8,43.0 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 40.2 ns | 42.9 ns | 36.8 ns | 40.7 ns |
| D38 | 15.4 ns | 42.6 ns | 74.5 ns | 75.4 ns | 108 ns |
| D57 | 16.9 ns | 36.9 ns | 83.2 ns | 374 ns | 340 ns |
| D76 | 16.6 ns | 96.7 ns | 371 ns | 322 ns | 610 ns |
| D115 | 22 ns | 84.2 ns | 391 ns | 796 ns | 1.21 µs |
| D153 | 24.5 ns | 389 ns | 648 ns | 966 ns | 1.59 µs |
| D230 | 28.9 ns | 424 ns | 712 ns | 1.21 µs | 2.84 µs |
| D307 | 43.2 ns | 742 ns | 1.75 µs | 2.81 µs | 4.01 µs |
| D462 | 61.6 ns | 868 ns | 3.34 µs | 5.38 µs | 7.86 µs |
| D616 | 68.1 ns | 1.75 µs | 5.66 µs | 7.73 µs | 12.9 µs |
| D924 | 102 ns | 3.24 µs | 10.3 µs | 22.1 µs | 20.8 µs |
| D1232 | 110 ns | 5.6 µs | 20.4 µs | 25.8 µs | 43 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.1 88.2,200.6 124.4,198.7 160.5,198.9 196.7,192.9 232.9,190.5 269.1,186.9 305.3,178.2 341.5,170.5 377.6,168.3 413.8,159.5 450.0,157.8 450.0,28.3 413.8,44.1 377.6,54.5 341.5,65.2 305.3,79.8 269.1,87.3 232.9,100.0 196.7,105.9 160.5,120.7 124.4,133.4 88.2,158.4 52.0,179.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.1 88.2,200.6 124.4,198.7 160.5,198.9 196.7,192.9 232.9,190.5 269.1,186.9 305.3,178.2 341.5,170.5 377.6,168.3 413.8,159.5 450.0,157.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.8 88.2,178.5 124.4,181.6 160.5,160.7 196.7,163.7 232.9,130.5 269.1,128.7 305.3,116.5 341.5,113.1 377.6,97.9 413.8,84.5 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,166.4 124.4,164.0 160.5,131.5 196.7,130.4 232.9,119.4 269.1,117.4 305.3,97.8 341.5,83.8 377.6,72.4 413.8,59.3 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.7 88.2,166.1 124.4,131.4 160.5,134.6 196.7,114.9 232.9,110.8 269.1,105.8 305.3,87.5 341.5,73.5 377.6,65.6 413.8,42.8 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,158.4 124.4,133.4 160.5,120.7 196.7,105.9 232.9,100.0 269.1,87.3 305.3,79.8 341.5,65.2 377.6,54.5 413.8,44.1 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.87 ns | 270 ns | 400 ns | 363 ns | 377 ns |
| D38 | 2.11 ns | 403 ns | 409 ns | 410 ns | 415 ns |
| D57 | 268 ns | 435 ns | 479 ns | 481 ns | 466 ns |
| D76 | 267 ns | 509 ns | 500 ns | 566 ns | 631 ns |
| D115 | 313 ns | 490 ns | 624 ns | 947 ns | 1.27 µs |
| D153 | 304 ns | 509 ns | 644 ns | 1.02 µs | 1.3 µs |
| D230 | 564 ns | 649 ns | 725 ns | 826 ns | 1.8 µs |
| D307 | 654 ns | 815 ns | 1.2 µs | 1.36 µs | 7.61 µs |
| D462 | 1.18 µs | 2.24 µs | 3.69 µs | 3.58 µs | 5.45 µs |
| D616 | 1.36 µs | 1.24 µs | 1.88 µs | 2.06 µs | 3.69 µs |
| D924 | 1.84 µs | 2.08 µs | 2.89 µs | 3.68 µs | 3.59 µs |
| D1232 | 3.24 µs | 3.17 µs | 4.5 µs | 5.52 µs | 6.25 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.6 88.2,193.8 124.4,88.6 160.5,88.6 196.7,85.2 232.9,85.9 269.1,72.4 305.3,69.2 341.5,56.5 377.6,53.3 413.8,46.8 450.0,34.5 450.0,20.2 413.8,32.3 377.6,31.7 341.5,23.2 305.3,15.9 269.1,47.2 232.9,54.4 196.7,54.7 160.5,70.0 124.4,76.6 88.2,79.1 52.0,81.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.6 88.2,193.8 124.4,88.6 160.5,88.6 196.7,85.2 232.9,85.9 269.1,72.4 305.3,69.2 341.5,56.5 377.6,53.3 413.8,46.8 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,88.4 88.2,79.7 124.4,78.1 160.5,74.7 196.7,75.5 232.9,74.7 269.1,69.4 305.3,64.4 341.5,42.5 377.6,55.3 413.8,44.1 450.0,35.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.9 88.2,79.4 124.4,76.0 160.5,75.1 196.7,70.3 232.9,69.6 269.1,67.0 305.3,56.0 341.5,31.7 377.6,46.3 413.8,36.9 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,79.4 124.4,75.9 160.5,72.3 196.7,61.2 232.9,59.6 269.1,64.1 305.3,53.3 341.5,32.3 377.6,44.3 413.8,31.7 450.0,22.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.2 88.2,79.1 124.4,76.6 160.5,70.0 196.7,54.7 232.9,54.4 269.1,47.2 305.3,15.9 341.5,23.2 377.6,31.7 413.8,32.3 450.0,20.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.88 µs | 3.1 µs | 6.07 µs | 5.44 µs | 6.95 µs |
| D38 | 2.89 µs | 6.07 µs | 7.64 µs | 9.02 µs | 10.5 µs |
| D57 | 2.47 µs | 2.26 µs | 2.76 µs | 2.71 µs | 2.21 µs |
| D76 | 2.29 µs | 2.98 µs | 2.78 µs | 2.5 µs | 3.02 µs |
| D115 | 4.74 µs | 5.05 µs | 5.23 µs | 5.48 µs | 7.22 µs |
| D153 | 4.55 µs | 5.01 µs | 5.3 µs | 5.82 µs | 6.27 µs |
| D230 | 6.44 µs | 7.1 µs | 5.12 µs | 5.51 µs | 9.85 µs |
| D307 | 9.54 µs | 11.8 µs | 13.5 µs | 14.3 µs | 11.2 µs |
| D462 | 9.49 µs | 8.7 µs | 15.4 µs | 14.7 µs | 19.8 µs |
| D616 | 16 µs | 19.1 µs | 29.8 µs | 25.9 µs | 41.3 µs |
| D924 | 20.5 µs | 39.4 µs | 55 µs | 70.9 µs | 65 µs |
| D1232 | 34.3 µs | 62.1 µs | 96.4 µs | 126 µs | 127 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.4 88.2,179.3 124.4,183.8 160.5,186.1 196.7,165.0 232.9,166.2 269.1,156.1 305.3,144.7 341.5,144.9 377.6,129.7 413.8,122.6 450.0,107.6 450.0,69.6 413.8,89.2 377.6,102.3 341.5,123.5 305.3,140.0 269.1,143.8 232.9,156.9 196.7,152.8 160.5,178.0 124.4,187.0 88.2,142.0 52.0,153.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.4 88.2,179.3 124.4,183.8 160.5,186.1 196.7,165.0 232.9,166.2 269.1,156.1 305.3,144.7 341.5,144.9 377.6,129.7 413.8,122.6 450.0,107.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.2 88.2,157.8 124.4,186.4 160.5,178.4 196.7,163.1 232.9,163.3 269.1,153.3 305.3,138.7 341.5,147.4 377.6,124.6 413.8,103.7 450.0,90.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.8 88.2,151.1 124.4,180.6 160.5,180.4 196.7,162.1 232.9,161.7 269.1,162.7 305.3,134.7 341.5,130.9 377.6,111.7 413.8,94.0 450.0,77.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.0 88.2,146.3 124.4,181.1 160.5,183.5 196.7,160.8 232.9,159.0 269.1,160.6 305.3,133.0 341.5,132.2 377.6,115.8 413.8,86.6 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.9 88.2,142.0 124.4,187.0 160.5,178.0 196.7,152.8 232.9,156.9 269.1,143.8 305.3,140.0 341.5,123.5 377.6,102.3 413.8,89.2 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.5 ns | 2.05 µs | 4.19 µs | 3.53 µs | 4.25 µs |
| D38 | 13.4 ns | 4.18 µs | 4.83 µs | 5.36 µs | 5.88 µs |
| D57 | 58.5 ns | 3.95 µs | 4.68 µs | 4.61 µs | 4.15 µs |
| D76 | 62 ns | 4.56 µs | 4.81 µs | 4.46 µs | 5.65 µs |
| D115 | 147 ns | 8.33 µs | 9.03 µs | 10.2 µs | 13.5 µs |
| D153 | 175 ns | 8.4 µs | 9.47 µs | 10.9 µs | 11.9 µs |
| D230 | 368 ns | 12.6 µs | 9.65 µs | 11.5 µs | 21.2 µs |
| D307 | 393 ns | 20.4 µs | 22.9 µs | 28.4 µs | 23.3 µs |
| D462 | 634 ns | 53.2 µs | 148 µs | 202 µs | 283 µs |
| D616 | 776 ns | 156 µs | 328 µs | 259 µs | 532 µs |
| D924 | 989 ns | 417 µs | 457 µs | 794 µs | 1.26 ms |
| D1232 | 1.7 µs | 762 µs | 861 µs | 2.3 ms | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.7 88.2,205.8 124.4,184.4 160.5,183.6 196.7,171.1 232.9,168.6 269.1,157.8 305.3,156.9 341.5,149.9 377.6,147.0 413.8,143.5 450.0,135.6 450.0,30.1 413.8,40.0 377.6,52.5 341.5,61.6 305.3,97.8 269.1,99.2 232.9,107.5 196.7,105.6 160.5,118.3 124.4,122.7 88.2,117.7 52.0,122.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.7 88.2,205.8 124.4,184.4 160.5,183.6 196.7,171.1 232.9,168.6 269.1,157.8 305.3,156.9 341.5,149.9 377.6,147.0 413.8,143.5 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,133.0 88.2,122.6 124.4,123.4 160.5,121.4 196.7,112.6 232.9,112.5 269.1,106.7 305.3,99.7 341.5,85.8 377.6,70.2 413.8,56.0 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.6 88.2,120.5 124.4,121.0 160.5,120.6 196.7,111.5 232.9,110.8 269.1,110.5 305.3,98.0 341.5,71.0 377.6,59.5 413.8,54.7 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,125.1 88.2,119.0 124.4,121.2 160.5,121.7 196.7,109.8 232.9,108.8 269.1,108.0 305.3,94.9 341.5,66.5 377.6,62.9 413.8,46.7 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.4 88.2,117.7 124.4,122.7 160.5,118.3 196.7,105.6 232.9,107.5 269.1,99.2 305.3,97.8 341.5,61.6 377.6,52.5 413.8,40.0 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.17 ns | 13.4 ns | 18.5 ns | 29.4 ns | 28.9 ns |
| D38 | 7.29 ns | 20.5 ns | 44.4 ns | 150 ns | 211 ns |
| D57 | 80.5 ns | 137 ns | 208 ns | 358 ns | 322 ns |
| D76 | 82.9 ns | 145 ns | 294 ns | 314 ns | 628 ns |
| D115 | 98.2 ns | 223 ns | 387 ns | 736 ns | 1.15 µs |
| D153 | 114 ns | 318 ns | 631 ns | 940 ns | 1.55 µs |
| D230 | 145 ns | 430 ns | 625 ns | 1.2 µs | 2.74 µs |
| D307 | 155 ns | 733 ns | 1.7 µs | 2.72 µs | 3.87 µs |
| D462 | 205 ns | 790 ns | 2.81 µs | 5.65 µs | 7.61 µs |
| D616 | 228 ns | 1.63 µs | 4.67 µs | 5.39 µs | 12.4 µs |
| D924 | 257 ns | 2.7 µs | 6.97 µs | 14.9 µs | 20 µs |
| D1232 | 434 ns | 4.81 µs | 13.9 µs | 26.4 µs | 42.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,175.5 124.4,133.8 160.5,133.3 196.7,130.3 232.9,127.7 269.1,123.6 305.3,122.4 341.5,117.5 377.6,115.7 413.8,113.6 450.0,104.5 450.0,24.9 413.8,38.0 377.6,46.2 341.5,54.7 305.3,66.5 269.1,72.5 232.9,82.4 196.7,87.6 160.5,98.1 124.4,109.7 88.2,117.1 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,175.5 124.4,133.8 160.5,133.3 196.7,130.3 232.9,127.7 269.1,123.6 305.3,122.4 341.5,117.5 377.6,115.7 413.8,113.6 450.0,104.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,157.5 124.4,124.5 160.5,123.6 196.7,116.1 232.9,109.9 269.1,104.7 305.3,95.4 341.5,94.1 377.6,81.5 413.8,72.7 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,144.1 124.4,117.3 160.5,111.3 196.7,106.5 232.9,98.0 269.1,98.2 305.3,80.8 341.5,72.1 377.6,63.2 413.8,56.3 450.0,44.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,122.9 124.4,107.8 160.5,110.1 196.7,95.3 232.9,91.1 269.1,86.9 305.3,72.6 341.5,59.9 377.6,60.7 413.8,43.1 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,117.1 124.4,109.7 160.5,98.1 196.7,87.6 232.9,82.4 269.1,72.5 305.3,66.5 341.5,54.7 377.6,46.2 413.8,38.0 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
