# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 65.2 ns | 132 ns | 160 ns | 254 ns | 263 ns |
| D38 | 80.7 ns | 136 ns | 195 ns | 378 ns | 400 ns |
| D57 | 189 ns | 282 ns | 646 ns | 1.06 µs | 599 ns |
| D76 | 132 ns | 412 ns | 807 ns | 1 µs | 1.96 µs |
| D115 | 226 ns | 596 ns | 1.19 µs | 2.1 µs | 3.14 µs |
| D153 | 251 ns | 871 ns | 1.33 µs | 3.22 µs | 2.75 µs |
| D230 | 291 ns | 1.12 µs | 3.26 µs | 4.32 µs | 8.77 µs |
| D307 | 316 ns | 1.76 µs | 3.15 µs | 8.84 µs | 15 µs |
| D462 | 383 ns | 3.42 µs | 8.97 µs | 17 µs | 26 µs |
| D616 | 444 ns | 3.35 µs | 14.6 µs | 30.9 µs | 43.7 µs |
| D924 | 602 ns | 9.61 µs | 19.3 µs | 69.9 µs | 115 µs |
| D1232 | 957 ns | 13.5 µs | 52 µs | 75.8 µs | 224 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.4 88.2,173.7 124.4,159.0 160.5,165.2 196.7,155.9 232.9,154.0 269.1,151.5 305.3,150.0 341.5,146.7 377.6,144.1 413.8,138.8 450.0,130.8 450.0,36.0 413.8,47.5 377.6,64.4 341.5,73.4 305.3,83.0 269.1,92.3 232.9,112.4 196.7,110.1 160.5,118.3 124.4,138.9 88.2,145.9 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.4 88.2,173.7 124.4,159.0 160.5,165.2 196.7,155.9 232.9,154.0 269.1,151.5 305.3,150.0 341.5,146.7 377.6,144.1 413.8,138.8 450.0,130.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.1 88.2,164.7 124.4,152.0 160.5,145.4 196.7,139.0 232.9,132.4 269.1,128.1 305.3,120.1 341.5,108.6 377.6,109.0 413.8,90.7 450.0,84.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,158.4 124.4,137.6 160.5,133.7 196.7,127.0 232.9,125.1 269.1,109.5 305.3,110.0 341.5,91.9 377.6,83.5 413.8,78.6 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,146.9 124.4,129.0 160.5,130.0 196.7,117.1 232.9,109.7 269.1,104.6 305.3,92.1 341.5,80.8 377.6,70.4 413.8,56.2 450.0,54.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,145.9 124.4,138.9 160.5,118.3 196.7,110.1 232.9,112.4 269.1,92.3 305.3,83.0 341.5,73.4 377.6,64.4 413.8,47.5 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.52 ns | 1.62 µs | 3.49 µs | 3.46 µs | 3.98 µs |
| D38 | 1.56 ns | 2.65 µs | 2.72 µs | 4.66 µs | 4.69 µs |
| D57 | 2.37 ns | 2.15 µs | 4.8 µs | 6.17 µs | 5.43 µs |
| D76 | 1.4 ns | 4.24 µs | 7.38 µs | 6.54 µs | 14.4 µs |
| D115 | 13.3 ns | 4.66 µs | 10.9 µs | 18.9 µs | 25.2 µs |
| D153 | 13.1 ns | 7.21 µs | 11.4 µs | 23.5 µs | 20 µs |
| D230 | 46 ns | 11.6 µs | 23.7 µs | 37.4 µs | 79.4 µs |
| D307 | 85.2 ns | 15.4 µs | 22.6 µs | 79 µs | 123 µs |
| D462 | 110 ns | 24 µs | 79.6 µs | 155 µs | 195 µs |
| D616 | 120 ns | 22.7 µs | 132 µs | 265 µs | 347 µs |
| D924 | 220 ns | 85.5 µs | 147 µs | 604 µs | 915 µs |
| D1232 | 370 ns | 115 µs | 414 µs | 648 µs | 2.81 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,204.5 124.4,199.3 160.5,205.8 196.7,177.9 232.9,178.0 269.1,162.5 305.3,154.8 341.5,151.7 377.6,150.6 413.8,143.1 450.0,136.6 450.0,25.8 413.8,39.7 377.6,51.7 341.5,58.9 305.3,64.6 269.1,70.0 232.9,87.1 196.7,84.2 160.5,91.2 124.4,103.3 88.2,105.1 52.0,107.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,204.5 124.4,199.3 160.5,205.8 196.7,177.9 232.9,178.0 269.1,162.5 305.3,154.8 341.5,151.7 377.6,150.6 413.8,143.1 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.3 88.2,112.2 124.4,114.8 160.5,106.4 196.7,105.2 232.9,99.8 269.1,93.9 305.3,90.4 341.5,84.9 377.6,85.5 413.8,69.1 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,111.9 124.4,104.8 160.5,99.5 196.7,94.6 232.9,94.0 269.1,85.0 305.3,85.6 341.5,70.0 377.6,63.7 413.8,62.4 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,105.2 124.4,101.7 160.5,101.0 196.7,87.8 232.9,85.1 269.1,79.3 305.3,70.1 341.5,61.7 377.6,55.0 413.8,44.8 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,105.1 124.4,103.3 160.5,91.2 196.7,84.2 232.9,87.1 269.1,70.0 305.3,64.6 341.5,58.9 377.6,51.7 413.8,39.7 450.0,25.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.8 ns | 39.6 ns | 42.9 ns | 40.6 ns | 40.1 ns |
| D38 | 16.1 ns | 37.2 ns | 55.7 ns | 75.2 ns | 94.3 ns |
| D57 | 16.5 ns | 26.8 ns | 95.7 ns | 401 ns | 262 ns |
| D76 | 10.8 ns | 75.9 ns | 367 ns | 307 ns | 753 ns |
| D115 | 22.1 ns | 76.4 ns | 342 ns | 845 ns | 1.01 µs |
| D153 | 15.6 ns | 393 ns | 487 ns | 1.03 µs | 975 ns |
| D230 | 29 ns | 358 ns | 1.14 µs | 1.57 µs | 2.83 µs |
| D307 | 42.7 ns | 718 ns | 1.11 µs | 2.81 µs | 5.29 µs |
| D462 | 62 ns | 1.22 µs | 3.18 µs | 5.69 µs | 6.23 µs |
| D616 | 68.7 ns | 1.28 µs | 5.71 µs | 10.2 µs | 10.8 µs |
| D924 | 104 ns | 3.46 µs | 6.49 µs | 24.4 µs | 23.2 µs |
| D1232 | 111 ns | 5.32 µs | 18.7 µs | 16.3 µs | 49 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.7 88.2,199.7 124.4,199.1 160.5,208.2 196.7,192.8 232.9,200.3 269.1,186.9 305.3,178.5 341.5,170.4 377.6,168.2 413.8,159.3 450.0,157.8 450.0,25.5 413.8,41.7 377.6,58.2 341.5,70.3 305.3,73.8 269.1,87.4 232.9,110.5 196.7,109.8 160.5,116.2 124.4,139.1 88.2,161.3 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.7 88.2,199.7 124.4,199.1 160.5,208.2 196.7,192.8 232.9,200.3 269.1,186.9 305.3,178.5 341.5,170.4 377.6,168.2 413.8,159.3 450.0,157.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.1 88.2,181.5 124.4,188.6 160.5,166.0 196.7,165.9 232.9,130.3 269.1,132.3 305.3,117.2 341.5,105.6 377.6,104.6 413.8,83.1 450.0,73.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,172.7 124.4,161.0 160.5,131.8 196.7,133.3 232.9,125.6 269.1,107.2 305.3,107.8 341.5,84.9 377.6,72.2 413.8,69.4 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.6 88.2,166.2 124.4,129.9 160.5,135.7 196.7,113.7 232.9,109.4 269.1,100.3 305.3,87.5 341.5,72.2 377.6,59.5 413.8,40.6 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,161.3 124.4,139.1 160.5,116.2 196.7,109.8 232.9,110.5 269.1,87.4 305.3,73.8 341.5,70.3 377.6,58.2 413.8,41.7 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.1 ns | 277 ns | 398 ns | 399 ns | 409 ns |
| D38 | 2.49 ns | 340 ns | 304 ns | 414 ns | 383 ns |
| D57 | 263 ns | 297 ns | 502 ns | 518 ns | 341 ns |
| D76 | 166 ns | 349 ns | 481 ns | 394 ns | 850 ns |
| D115 | 316 ns | 355 ns | 459 ns | 1.05 µs | 1.1 µs |
| D153 | 223 ns | 527 ns | 432 ns | 1.12 µs | 800 ns |
| D230 | 561 ns | 559 ns | 1.13 µs | 1.26 µs | 1.82 µs |
| D307 | 743 ns | 722 ns | 772 ns | 1.32 µs | 11.1 µs |
| D462 | 1.16 µs | 3.14 µs | 3.36 µs | 4.05 µs | 4.39 µs |
| D616 | 1.21 µs | 1.09 µs | 1.95 µs | 2.91 µs | 3.31 µs |
| D924 | 2.19 µs | 2.25 µs | 1.61 µs | 3.95 µs | 4.81 µs |
| D1232 | 3.23 µs | 2.51 µs | 4.21 µs | 3.26 µs | 7.06 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.4 88.2,194.1 124.4,113.2 160.5,121.2 196.7,110.0 232.9,116.1 269.1,100.0 305.3,95.2 341.5,87.4 377.6,86.7 413.8,76.4 450.0,69.6 450.0,56.1 413.8,62.7 377.6,69.2 341.5,64.3 305.3,48.2 269.1,79.6 232.9,93.9 196.7,88.3 160.5,92.8 124.4,108.7 88.2,106.7 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.4 88.2,194.1 124.4,113.2 160.5,121.2 196.7,110.0 232.9,116.1 269.1,100.0 305.3,95.2 341.5,87.4 377.6,86.7 413.8,76.4 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.3 88.2,108.7 124.4,111.1 160.5,108.3 196.7,108.0 232.9,101.1 269.1,100.1 305.3,95.7 341.5,70.2 377.6,88.6 413.8,75.9 450.0,74.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.0 88.2,110.7 124.4,102.0 160.5,102.7 196.7,103.5 232.9,104.6 269.1,88.0 305.3,94.5 341.5,69.0 377.6,78.4 413.8,81.8 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.0 88.2,105.3 124.4,101.4 160.5,106.2 196.7,89.1 232.9,88.1 269.1,86.0 305.3,85.2 341.5,65.7 377.6,71.5 413.8,66.1 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,106.7 124.4,108.7 160.5,92.8 196.7,88.3 232.9,93.9 269.1,79.6 305.3,48.2 341.5,64.3 377.6,69.2 413.8,62.7 450.0,56.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.22 µs | 3.13 µs | 6.07 µs | 6.38 µs | 7.3 µs |
| D38 | 2.75 µs | 4.72 µs | 5.32 µs | 9.03 µs | 9.91 µs |
| D57 | 2.27 µs | 1.35 µs | 2.94 µs | 2.98 µs | 1.56 µs |
| D76 | 1.42 µs | 1.78 µs | 2.74 µs | 1.89 µs | 3.86 µs |
| D115 | 4.84 µs | 3.74 µs | 4.14 µs | 5.97 µs | 6.2 µs |
| D153 | 3.55 µs | 4.91 µs | 3.95 µs | 6.22 µs | 3.73 µs |
| D230 | 6.6 µs | 6.24 µs | 8.94 µs | 7.74 µs | 9.96 µs |
| D307 | 9.78 µs | 10.9 µs | 7.92 µs | 14.1 µs | 16.6 µs |
| D462 | 9.73 µs | 13.1 µs | 14.3 µs | 16.4 µs | 15.5 µs |
| D616 | 11.6 µs | 15.3 µs | 30.1 µs | 35.4 µs | 36.6 µs |
| D924 | 25 µs | 42.8 µs | 32.9 µs | 76.8 µs | 84.8 µs |
| D1232 | 33.9 µs | 56.5 µs | 89 µs | 77.5 µs | 156 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.9 88.2,180.7 124.4,186.3 160.5,199.8 196.7,164.3 232.9,173.3 269.1,155.4 305.3,144.0 341.5,144.1 377.6,139.1 413.8,116.8 450.0,108.0 450.0,63.8 413.8,81.4 377.6,105.8 341.5,130.6 305.3,128.6 269.1,143.4 232.9,171.9 196.7,157.2 160.5,170.9 124.4,197.1 88.2,143.6 52.0,152.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.9 88.2,180.7 124.4,186.3 160.5,199.8 196.7,164.3 232.9,173.3 269.1,155.4 305.3,144.0 341.5,144.1 377.6,139.1 413.8,116.8 450.0,108.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.9 88.2,165.1 124.4,201.3 160.5,193.4 196.7,171.8 232.9,163.9 269.1,157.0 305.3,140.8 341.5,135.5 377.6,131.0 413.8,101.2 450.0,93.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.8 88.2,161.6 124.4,178.8 160.5,180.8 196.7,168.9 232.9,170.2 269.1,146.6 305.3,150.1 341.5,133.0 377.6,111.4 413.8,108.8 450.0,80.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.3 88.2,146.3 124.4,178.4 160.5,191.5 196.7,158.3 232.9,157.1 269.1,150.7 305.3,133.3 341.5,129.0 377.6,106.7 413.8,84.3 450.0,84.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.4 88.2,143.6 124.4,197.1 160.5,170.9 196.7,157.2 232.9,171.9 269.1,143.4 305.3,128.6 341.5,130.6 377.6,105.8 413.8,81.4 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.4 ns | 2.04 µs | 4.19 µs | 4.09 µs | 4.67 µs |
| D38 | 13.4 ns | 3.26 µs | 3.23 µs | 5.35 µs | 5.35 µs |
| D57 | 56.6 ns | 2.38 µs | 4.81 µs | 5.12 µs | 2.88 µs |
| D76 | 57 ns | 2.88 µs | 4.68 µs | 3.55 µs | 7.15 µs |
| D115 | 145 ns | 6.13 µs | 7.41 µs | 10.9 µs | 11.6 µs |
| D153 | 129 ns | 8.4 µs | 7.14 µs | 11.7 µs | 7.04 µs |
| D230 | 361 ns | 11 µs | 16.5 µs | 16.1 µs | 21.5 µs |
| D307 | 468 ns | 19 µs | 13.5 µs | 28.5 µs | 34.5 µs |
| D462 | 654 ns | 77.3 µs | 138 µs | 205 µs | 223 µs |
| D616 | 638 ns | 119 µs | 325 µs | 352 µs | 449 µs |
| D924 | 1.13 µs | 450 µs | 253 µs | 846 µs | 1.62 ms |
| D1232 | 1.73 µs | 712 µs | 801 µs | 1.49 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,206.9 88.2,205.8 124.4,184.9 160.5,184.8 196.7,171.2 232.9,173.0 269.1,158.1 305.3,154.3 341.5,149.5 377.6,149.8 413.8,141.6 450.0,135.4 450.0,27.5 413.8,36.3 377.6,54.9 341.5,65.0 305.3,92.1 269.1,98.9 232.9,115.1 196.7,107.9 160.5,114.9 124.4,128.0 88.2,119.0 52.0,121.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,206.9 88.2,205.8 124.4,184.9 160.5,184.8 196.7,171.2 232.9,173.0 269.1,158.1 305.3,154.3 341.5,149.5 377.6,149.8 413.8,141.6 450.0,135.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,133.0 88.2,126.2 124.4,130.8 160.5,128.0 196.7,117.1 232.9,112.5 269.1,108.6 305.3,100.7 341.5,80.4 377.6,74.1 413.8,54.9 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.6 88.2,126.4 124.4,120.6 160.5,121.0 196.7,114.3 232.9,114.9 269.1,102.8 305.3,105.7 341.5,72.0 377.6,59.6 413.8,63.3 450.0,46.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.9 88.2,119.0 124.4,119.7 160.5,125.0 196.7,108.7 232.9,107.7 269.1,103.1 305.3,94.9 341.5,66.3 377.6,58.4 413.8,45.8 450.0,37.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.0 88.2,119.0 124.4,128.0 160.5,114.9 196.7,107.9 232.9,115.1 269.1,98.9 305.3,92.1 341.5,65.0 377.6,54.9 413.8,36.3 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.49 ns | 13.3 ns | 18.5 ns | 29.5 ns | 29.3 ns |
| D38 | 7.09 ns | 18.4 ns | 25 ns | 151 ns | 194 ns |
| D57 | 80.6 ns | 96.8 ns | 220 ns | 393 ns | 250 ns |
| D76 | 56.5 ns | 109 ns | 290 ns | 291 ns | 734 ns |
| D115 | 97.8 ns | 171 ns | 331 ns | 789 ns | 988 ns |
| D153 | 91.7 ns | 320 ns | 475 ns | 989 ns | 1.01 µs |
| D230 | 144 ns | 360 ns | 1.03 µs | 1.5 µs | 2.71 µs |
| D307 | 163 ns | 678 ns | 1.11 µs | 2.74 µs | 5.17 µs |
| D462 | 200 ns | 1.13 µs | 2.64 µs | 5.81 µs | 5.95 µs |
| D616 | 242 ns | 1.34 µs | 4.67 µs | 6.82 µs | 10.6 µs |
| D924 | 265 ns | 2.85 µs | 3.88 µs | 16.2 µs | 22.9 µs |
| D1232 | 441 ns | 4.58 µs | 13.1 µs | 17.1 µs | 48.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.9 88.2,176.0 124.4,133.7 160.5,139.9 196.7,130.4 232.9,131.5 269.1,123.7 305.3,121.5 341.5,118.0 377.6,114.7 413.8,113.1 450.0,104.2 450.0,22.7 413.8,35.6 377.6,49.0 341.5,59.0 305.3,61.4 269.1,72.7 232.9,89.9 196.7,90.2 160.5,95.4 124.4,114.1 88.2,118.5 52.0,151.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.9 88.2,176.0 124.4,133.7 160.5,139.9 196.7,130.4 232.9,131.5 269.1,123.7 305.3,121.5 341.5,118.0 377.6,114.7 413.8,113.1 450.0,104.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,159.4 124.4,130.6 160.5,128.5 196.7,120.7 232.9,109.8 269.1,107.8 305.3,96.8 341.5,87.9 377.6,84.9 413.8,71.8 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,154.1 124.4,116.3 160.5,111.5 196.7,109.2 232.9,102.9 269.1,89.5 305.3,88.2 341.5,73.1 377.6,63.2 413.8,66.4 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.2 88.2,122.8 124.4,106.2 160.5,111.5 196.7,94.1 232.9,90.2 269.1,82.9 305.3,72.5 341.5,59.4 377.6,56.7 413.8,41.6 450.0,40.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,118.5 124.4,114.1 160.5,95.4 196.7,90.2 232.9,89.9 269.1,72.7 305.3,61.4 341.5,59.0 377.6,49.0 413.8,35.6 450.0,22.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
