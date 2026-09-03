# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 131 ns | 139 ns | 130 ns | 301 ns | 389 ns |
| D38 | 251 ns | 241 ns | 536 ns | 405 ns | 622 ns |
| D57 | 341 ns | 593 ns | 704 ns | 592 ns | 1.14 µs |
| D76 | 521 ns | 568 ns | 1.1 µs | 1.37 µs | 1.9 µs |
| D115 | 222 ns | 766 ns | 2.36 µs | 3.69 µs | 5.22 µs |
| D153 | 253 ns | 851 ns | 3.5 µs | 5.6 µs | 5.5 µs |
| D230 | 291 ns | 1.31 µs | 8.27 µs | 10.5 µs | 13.8 µs |
| D307 | 242 ns | 1.07 µs | 11.7 µs | 12.9 µs | 18.2 µs |
| D462 | 394 ns | 3.38 µs | 24.3 µs | 33.3 µs | 51.8 µs |
| D616 | 485 ns | 4 µs | 37.1 µs | 39.6 µs | 85.2 µs |
| D924 | 345 ns | 9.66 µs | 81.6 µs | 121 µs | 131 µs |
| D1232 | 799 ns | 15.5 µs | 122 µs | 227 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.2 88.2,190.0 124.4,183.4 160.5,174.2 196.7,192.7 232.9,189.9 269.1,186.8 305.3,190.8 341.5,180.2 377.6,175.7 413.8,183.1 450.0,164.9 450.0,33.1 413.8,54.2 377.6,63.5 341.5,74.3 305.3,97.0 269.1,103.0 232.9,123.0 196.7,124.1 160.5,146.1 124.4,157.2 88.2,170.3 52.0,180.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.2 88.2,190.0 124.4,183.4 160.5,174.2 196.7,192.7 232.9,189.9 269.1,186.8 305.3,190.8 341.5,180.2 377.6,175.7 413.8,183.1 450.0,164.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,202.9 88.2,190.9 124.4,171.4 160.5,172.3 196.7,165.8 232.9,163.5 269.1,154.1 305.3,158.5 341.5,133.5 377.6,129.9 413.8,110.7 450.0,100.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,204.4 88.2,173.5 124.4,167.6 160.5,157.8 196.7,141.3 232.9,132.8 269.1,114.1 305.3,106.5 341.5,90.7 377.6,81.5 413.8,64.4 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.0 88.2,179.7 124.4,171.4 160.5,153.2 196.7,131.7 232.9,122.6 269.1,108.9 305.3,104.5 341.5,83.9 377.6,80.1 413.8,55.9 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.5 88.2,170.3 124.4,157.2 160.5,146.1 196.7,124.1 232.9,123.0 269.1,103.0 305.3,97.0 341.5,74.3 377.6,63.5 413.8,54.2 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.79 µs | 2.71 µs | 3.75 µs | 3.73 µs |
| D38 | 2.11 ns | 2.35 µs | 3.77 µs | 3.08 µs | 3.95 µs |
| D57 | 2.81 ns | 3.83 µs | 4.38 µs | 3.3 µs | 9.09 µs |
| D76 | 3.16 ns | 4.87 µs | 7.77 µs | 10.2 µs | 11.4 µs |
| D115 | 11.9 ns | 6.66 µs | 13.4 µs | 18.2 µs | 22.4 µs |
| D153 | 16.2 ns | 6.69 µs | 16.1 µs | 22.2 µs | 30.2 µs |
| D230 | 46.1 ns | 11.4 µs | 23.8 µs | 45.9 µs | 78.1 µs |
| D307 | 45 ns | 9.17 µs | 38.6 µs | 61.1 µs | 110 µs |
| D462 | 111 ns | 23.9 µs | 80.8 µs | 155 µs | 247 µs |
| D616 | 149 ns | 30.2 µs | 133 µs | 193 µs | 446 µs |
| D924 | 100 ns | 85.5 µs | 266 µs | 557 µs | 711 µs |
| D1232 | 273 ns | 132 µs | 404 µs | 916 µs | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,200.7 124.4,197.2 160.5,195.7 196.7,179.3 232.9,175.4 269.1,162.5 305.3,162.8 341.5,151.6 377.6,147.9 413.8,152.8 450.0,140.4 450.0,25.7 413.8,42.8 377.6,48.6 341.5,55.9 305.3,65.9 269.1,70.2 232.9,82.0 196.7,85.7 160.5,94.1 124.4,96.9 88.2,107.2 52.0,108.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,200.7 124.4,197.2 160.5,195.7 196.7,179.3 232.9,175.4 269.1,162.5 305.3,162.8 341.5,151.6 377.6,147.9 413.8,152.8 450.0,140.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.0 88.2,113.7 124.4,107.6 160.5,104.6 196.7,100.8 232.9,100.7 269.1,94.1 305.3,96.8 341.5,84.9 377.6,82.0 413.8,69.1 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.9 88.2,107.8 124.4,106.0 160.5,98.8 196.7,92.1 232.9,89.8 269.1,85.0 305.3,79.0 341.5,69.8 377.6,63.6 413.8,55.0 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,110.3 124.4,109.5 160.5,95.5 196.7,88.3 232.9,85.8 269.1,76.8 305.3,73.3 341.5,61.7 377.6,59.0 413.8,45.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,107.2 124.4,96.9 160.5,94.1 196.7,85.7 232.9,82.0 269.1,70.2 305.3,65.9 341.5,55.9 377.6,48.6 413.8,42.8 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 39.7 ns | 39.4 ns | 40.3 ns | 40.2 ns |
| D38 | 15.5 ns | 32.2 ns | 67.3 ns | 56 ns | 97.3 ns |
| D57 | 17.6 ns | 41.1 ns | 67 ns | 259 ns | 389 ns |
| D76 | 17.4 ns | 64.9 ns | 391 ns | 403 ns | 618 ns |
| D115 | 22.2 ns | 71.7 ns | 392 ns | 781 ns | 962 ns |
| D153 | 25.4 ns | 386 ns | 683 ns | 971 ns | 1.31 µs |
| D230 | 28.6 ns | 372 ns | 1.13 µs | 2.01 µs | 2.82 µs |
| D307 | 19.5 ns | 416 ns | 1.77 µs | 2.42 µs | 4.92 µs |
| D462 | 61.7 ns | 1.22 µs | 3.17 µs | 5.7 µs | 7.87 µs |
| D616 | 86.2 ns | 1.72 µs | 5.46 µs | 7.64 µs | 13.8 µs |
| D924 | 71.4 ns | 3.46 µs | 10.4 µs | 22.3 µs | 20.6 µs |
| D1232 | 88.6 ns | 5.61 µs | 17 µs | 23.8 µs | 49.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,200.5 124.4,197.7 160.5,198.0 196.7,192.7 232.9,189.7 269.1,187.2 305.3,195.5 341.5,170.5 377.6,163.2 413.8,167.3 450.0,162.6 450.0,25.4 413.8,44.3 377.6,53.0 341.5,65.2 305.3,75.4 269.1,87.5 232.9,104.2 196.7,110.9 160.5,120.4 124.4,130.5 88.2,160.6 52.0,179.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,200.5 124.4,197.7 160.5,198.0 196.7,192.7 232.9,189.7 269.1,187.2 305.3,195.5 341.5,170.5 377.6,163.2 413.8,167.3 450.0,162.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,184.6 124.4,179.3 160.5,169.4 196.7,167.2 232.9,130.7 269.1,131.5 305.3,129.0 341.5,105.8 377.6,98.2 413.8,83.0 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.2 88.2,168.6 124.4,168.7 160.5,130.4 196.7,130.4 232.9,118.3 269.1,107.3 305.3,97.7 341.5,84.9 377.6,73.1 413.8,59.2 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,172.6 124.4,139.3 160.5,129.7 196.7,115.4 232.9,110.6 269.1,94.8 305.3,90.8 341.5,72.2 377.6,65.9 413.8,42.6 450.0,41.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,160.6 124.4,130.5 160.5,120.4 196.7,110.9 232.9,104.2 269.1,87.5 305.3,75.4 341.5,65.2 377.6,53.0 413.8,44.3 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 292 ns | 343 ns | 406 ns | 382 ns |
| D38 | 2.11 ns | 303 ns | 382 ns | 314 ns | 377 ns |
| D57 | 278 ns | 469 ns | 467 ns | 299 ns | 604 ns |
| D76 | 285 ns | 392 ns | 511 ns | 699 ns | 615 ns |
| D115 | 279 ns | 498 ns | 626 ns | 961 ns | 973 ns |
| D153 | 300 ns | 487 ns | 770 ns | 990 ns | 1.23 µs |
| D230 | 551 ns | 517 ns | 1.13 µs | 1.5 µs | 1.79 µs |
| D307 | 395 ns | 394 ns | 1.29 µs | 1.22 µs | 9.09 µs |
| D462 | 1.21 µs | 3.14 µs | 3.34 µs | 3.97 µs | 5.38 µs |
| D616 | 1.56 µs | 1.39 µs | 1.92 µs | 2.05 µs | 3.95 µs |
| D924 | 1.31 µs | 2.15 µs | 2.86 µs | 3.62 µs | 3.55 µs |
| D1232 | 2.54 µs | 3.14 µs | 3.47 µs | 5.1 µs | 7.23 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.0 88.2,193.8 124.4,87.8 160.5,87.3 196.7,87.7 232.9,86.1 269.1,72.9 305.3,80.2 341.5,55.8 377.6,50.3 413.8,54.2 450.0,39.8 450.0,17.0 413.8,32.5 377.6,30.1 341.5,23.5 305.3,12.1 269.1,47.4 232.9,55.5 196.7,60.6 160.5,70.5 124.4,70.9 88.2,81.2 52.0,80.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.0 88.2,193.8 124.4,87.8 160.5,87.3 196.7,87.7 232.9,86.1 269.1,72.9 305.3,80.2 341.5,55.8 377.6,50.3 413.8,54.2 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.8 88.2,85.9 124.4,76.5 160.5,80.3 196.7,75.1 232.9,75.6 269.1,74.3 305.3,80.2 341.5,35.1 377.6,52.9 413.8,43.4 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.2 88.2,80.9 124.4,76.5 160.5,74.6 196.7,70.2 232.9,65.7 269.1,57.3 305.3,54.5 341.5,33.8 377.6,45.8 413.8,37.2 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.6 88.2,85.1 124.4,86.2 160.5,67.8 196.7,60.9 232.9,60.2 269.1,51.2 305.3,55.7 341.5,30.1 377.6,44.4 413.8,32.1 450.0,24.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.9 88.2,81.2 124.4,70.9 160.5,70.5 196.7,60.6 232.9,55.5 269.1,47.4 305.3,12.1 341.5,23.5 377.6,30.1 413.8,32.5 450.0,17.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.75 µs | 3.26 µs | 4.77 µs | 6.65 µs | 6.99 µs |
| D38 | 2.88 µs | 4.21 µs | 7.24 µs | 6.41 µs | 8.83 µs |
| D57 | 2.8 µs | 2.58 µs | 2.71 µs | 1.44 µs | 2.87 µs |
| D76 | 2.8 µs | 2.01 µs | 2.96 µs | 3.27 µs | 3 µs |
| D115 | 4.51 µs | 4.94 µs | 5.17 µs | 5.51 µs | 5.73 µs |
| D153 | 4.61 µs | 4.5 µs | 5.73 µs | 5.74 µs | 5.54 µs |
| D230 | 6.65 µs | 5.92 µs | 8.85 µs | 9.46 µs | 9.86 µs |
| D307 | 6.03 µs | 6.24 µs | 13.6 µs | 10.8 µs | 13.9 µs |
| D462 | 9.72 µs | 13 µs | 14.2 µs | 16.5 µs | 20 µs |
| D616 | 17 µs | 19.2 µs | 30.1 µs | 25.9 µs | 45.6 µs |
| D924 | 15.8 µs | 42.6 µs | 55.6 µs | 70.7 µs | 67.3 µs |
| D1232 | 27.6 µs | 61.9 µs | 81.6 µs | 115 µs | 162 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.7 88.2,179.3 124.4,180.2 160.5,180.2 196.7,166.4 232.9,165.7 269.1,155.2 305.3,158.0 341.5,144.2 377.6,128.0 413.8,130.1 450.0,113.9 450.0,62.7 413.8,88.1 377.6,99.4 341.5,123.3 305.3,133.9 269.1,143.8 232.9,160.4 196.7,159.5 160.5,178.1 124.4,179.5 88.2,146.9 52.0,153.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.7 88.2,179.3 124.4,180.2 160.5,180.2 196.7,166.4 232.9,165.7 269.1,155.2 305.3,158.0 341.5,144.2 377.6,128.0 413.8,130.1 450.0,113.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.8 88.2,168.4 124.4,182.6 160.5,189.8 196.7,163.7 232.9,166.5 269.1,158.5 305.3,157.0 341.5,135.6 377.6,124.5 413.8,101.4 450.0,90.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.7 88.2,152.7 124.4,181.2 160.5,178.6 196.7,162.4 232.9,159.4 269.1,146.9 305.3,134.5 341.5,133.1 377.6,111.5 413.8,93.7 450.0,82.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,156.2 124.4,199.4 160.5,175.7 196.7,160.6 232.9,159.4 269.1,144.9 305.3,141.2 341.5,128.9 377.6,115.7 413.8,86.7 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,146.9 124.4,179.5 160.5,178.1 196.7,159.5 232.9,160.4 269.1,143.8 305.3,133.9 341.5,123.3 377.6,99.4 413.8,88.1 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 2.25 µs | 3.31 µs | 4.41 µs | 4.34 µs |
| D38 | 6.48 ns | 2.87 µs | 4.39 µs | 3.63 µs | 4.6 µs |
| D57 | 62.4 ns | 4.17 µs | 4.55 µs | 2.84 µs | 5.13 µs |
| D76 | 83 ns | 3.28 µs | 5.08 µs | 5.73 µs | 5.57 µs |
| D115 | 137 ns | 8.25 µs | 9.02 µs | 10.4 µs | 10.8 µs |
| D153 | 178 ns | 7.86 µs | 10.3 µs | 11 µs | 10.4 µs |
| D230 | 344 ns | 10.9 µs | 16.4 µs | 19.8 µs | 21.3 µs |
| D307 | 219 ns | 11.3 µs | 22.9 µs | 22.3 µs | 30.1 µs |
| D462 | 638 ns | 78.3 µs | 137 µs | 206 µs | 285 µs |
| D616 | 820 ns | 157 µs | 326 µs | 257 µs | 573 µs |
| D924 | 698 ns | 452 µs | 459 µs | 795 µs | 1.28 ms |
| D1232 | 1.2 µs | 765 µs | 784 µs | 2.12 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.8 124.4,158.7 160.5,155.2 196.7,149.0 232.9,145.7 269.1,137.5 305.3,143.1 341.5,129.9 377.6,126.7 413.8,128.8 450.0,122.0 450.0,25.0 413.8,35.5 377.6,45.5 341.5,54.1 305.3,82.0 269.1,86.3 232.9,95.2 196.7,94.7 160.5,103.0 124.4,104.0 88.2,105.3 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.8 124.4,158.7 160.5,155.2 196.7,149.0 232.9,145.7 269.1,137.5 305.3,143.1 341.5,129.9 377.6,126.7 413.8,128.8 450.0,122.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.2 88.2,111.2 124.4,106.6 160.5,109.5 196.7,98.1 232.9,98.7 269.1,94.6 305.3,94.2 341.5,70.2 377.6,61.5 413.8,48.4 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,105.9 124.4,105.5 160.5,104.1 196.7,97.0 232.9,95.4 269.1,89.5 305.3,85.4 341.5,63.2 377.6,52.5 413.8,48.2 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,108.3 124.4,111.4 160.5,102.6 196.7,95.3 232.9,94.5 269.1,87.2 305.3,85.8 341.5,58.2 377.6,55.4 413.8,41.4 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,105.3 124.4,104.0 160.5,103.0 196.7,94.7 232.9,95.2 269.1,86.3 305.3,82.0 341.5,54.1 377.6,45.5 413.8,35.5 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.76 ns | 13.5 ns | 18.2 ns | 29.7 ns | 28.9 ns |
| D38 | 7.29 ns | 16.7 ns | 31.3 ns | 239 ns | 403 ns |
| D57 | 176 ns | 207 ns | 491 ns | 412 ns | 768 ns |
| D76 | 208 ns | 205 ns | 647 ns | 725 ns | 1.22 µs |
| D115 | 95.2 ns | 653 ns | 992 ns | 1.52 µs | 1.68 µs |
| D153 | 106 ns | 836 ns | 1.51 µs | 1.88 µs | 2.1 µs |
| D230 | 148 ns | 363 ns | 2.3 µs | 3.43 µs | 4.21 µs |
| D307 | 119 ns | 1.3 µs | 3.64 µs | 3.6 µs | 6.2 µs |
| D462 | 176 ns | 3.76 µs | 5.84 µs | 9.27 µs | 11.5 µs |
| D616 | 256 ns | 1.64 µs | 10.5 µs | 9.2 µs | 20.5 µs |
| D924 | 169 ns | 2.86 µs | 16.7 µs | 25.5 µs | 25.8 µs |
| D1232 | 340 ns | 16 µs | 24.1 µs | 38.4 µs | 61.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,175.5 124.4,120.2 160.5,117.3 196.7,130.9 232.9,128.9 269.1,123.2 305.3,127.0 341.5,120.2 377.6,113.6 413.8,120.9 450.0,108.8 450.0,18.4 413.8,33.5 377.6,37.5 341.5,47.6 305.3,58.3 269.1,65.0 232.9,77.1 196.7,81.0 160.5,86.6 124.4,94.6 88.2,105.8 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,175.5 124.4,120.2 160.5,117.3 196.7,130.9 232.9,128.9 269.1,123.2 305.3,127.0 341.5,120.2 377.6,113.6 413.8,120.9 450.0,108.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.8 88.2,161.1 124.4,117.4 160.5,117.5 196.7,97.4 232.9,93.1 269.1,107.6 305.3,85.5 341.5,67.0 377.6,81.4 413.8,71.7 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.6 88.2,150.2 124.4,102.4 160.5,97.6 196.7,90.1 232.9,82.9 269.1,75.5 305.3,67.5 341.5,59.3 377.6,49.2 413.8,41.1 450.0,34.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,114.8 124.4,105.4 160.5,95.6 196.7,82.8 232.9,79.0 269.1,68.6 305.3,67.7 341.5,51.3 377.6,51.5 413.8,33.7 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,105.8 124.4,94.6 160.5,86.6 196.7,81.0 232.9,77.1 269.1,65.0 305.3,58.3 341.5,47.6 377.6,37.5 413.8,33.5 450.0,18.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
