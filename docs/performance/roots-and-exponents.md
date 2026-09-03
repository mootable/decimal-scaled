# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 96.8 ns | 133 ns | 159 ns | 192 ns | 314 ns |
| D38 | 237 ns | 266 ns | 462 ns | 600 ns | 837 ns |
| D57 | 326 ns | 280 ns | 500 ns | 1.13 µs | 1.13 µs |
| D76 | 447 ns | 792 ns | 1.19 µs | 1.38 µs | 1.91 µs |
| D115 | 200 ns | 507 ns | 2.42 µs | 2.59 µs | 4.39 µs |
| D153 | 172 ns | 825 ns | 3.48 µs | 5.64 µs | 6.62 µs |
| D230 | 247 ns | 1.2 µs | 8.11 µs | 9.87 µs | 13.6 µs |
| D307 | 275 ns | 1.34 µs | 11.7 µs | 10 µs | 12.6 µs |
| D462 | 367 ns | 3.09 µs | 24.3 µs | 35.5 µs | 47.2 µs |
| D616 | 432 ns | 4.71 µs | 37.3 µs | 57.9 µs | 66.5 µs |
| D924 | 496 ns | 9.06 µs | 60.1 µs | 78.8 µs | 198 µs |
| D1232 | 1.1 µs | 15.7 µs | 118 µs | 195 µs | 222 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.6 88.2,155.0 124.4,149.5 160.5,144.0 196.7,158.0 232.9,160.6 269.1,154.3 305.3,152.4 341.5,147.4 377.6,144.6 413.8,142.2 450.0,128.3 450.0,36.1 413.8,38.1 377.6,57.1 341.5,63.0 305.3,85.9 269.1,84.7 232.9,97.2 196.7,104.3 160.5,118.7 124.4,127.8 88.2,133.1 52.0,150.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.6 88.2,155.0 124.4,149.5 160.5,144.0 196.7,158.0 232.9,160.6 269.1,154.3 305.3,152.4 341.5,147.4 377.6,144.6 413.8,142.2 450.0,128.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.1 88.2,153.0 124.4,152.1 160.5,134.1 196.7,141.8 232.9,133.3 269.1,126.9 305.3,124.9 341.5,110.4 377.6,103.1 413.8,91.7 450.0,82.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.9 88.2,143.4 124.4,142.0 160.5,127.0 196.7,114.6 232.9,108.3 269.1,93.6 305.3,87.3 341.5,74.6 377.6,67.1 413.8,58.8 450.0,47.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,138.9 124.4,127.9 160.5,124.4 196.7,113.5 232.9,100.0 269.1,90.2 305.3,89.9 341.5,68.0 377.6,59.5 413.8,54.1 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.1 88.2,133.1 124.4,127.8 160.5,118.7 196.7,104.3 232.9,97.2 269.1,84.7 305.3,85.9 341.5,63.0 377.6,57.1 413.8,38.1 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.14 ns | 1.68 µs | 3.56 µs | 2.09 µs | 3.87 µs |
| D38 | 1.56 ns | 3.24 µs | 4.21 µs | 4.32 µs | 4.74 µs |
| D57 | 2.18 ns | 2.07 µs | 3.67 µs | 5.72 µs | 9.2 µs |
| D76 | 2.86 ns | 5.9 µs | 7.04 µs | 10.3 µs | 11.4 µs |
| D115 | 11.8 ns | 3.93 µs | 13.5 µs | 14 µs | 20.5 µs |
| D153 | 11 ns | 7.39 µs | 15.4 µs | 24.2 µs | 35.2 µs |
| D230 | 34.8 ns | 11.4 µs | 23.7 µs | 42.2 µs | 78.6 µs |
| D307 | 68.2 ns | 11.8 µs | 38.1 µs | 42.9 µs | 63 µs |
| D462 | 131 ns | 22.3 µs | 79.7 µs | 164 µs | 229 µs |
| D616 | 114 ns | 38.5 µs | 130 µs | 267 µs | 348 µs |
| D924 | 181 ns | 80.7 µs | 196 µs | 410 µs | 988 µs |
| D1232 | 373 ns | 132 µs | 351 µs | 772 µs | 2.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.6 88.2,204.5 124.4,200.3 160.5,197.0 196.7,179.4 232.9,180.2 269.1,165.9 305.3,157.6 341.5,149.5 377.6,151.2 413.8,145.5 450.0,136.5 450.0,28.3 413.8,38.7 377.6,51.7 341.5,56.8 305.3,72.9 269.1,70.1 232.9,80.1 196.7,86.8 160.5,94.1 124.4,96.7 88.2,105.0 52.0,107.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.6 88.2,204.5 124.4,200.3 160.5,197.0 196.7,179.4 232.9,180.2 269.1,165.9 305.3,157.6 341.5,149.5 377.6,151.2 413.8,145.5 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.9 88.2,109.7 124.4,115.2 160.5,102.3 196.7,107.3 232.9,99.5 269.1,94.1 305.3,93.7 341.5,85.8 377.6,79.0 413.8,69.8 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,106.4 124.4,108.2 160.5,100.1 196.7,91.9 232.9,90.3 269.1,85.0 305.3,79.1 341.5,70.0 377.6,63.9 413.8,58.8 450.0,51.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.1 88.2,106.1 124.4,102.6 160.5,95.3 196.7,91.5 232.9,84.8 269.1,77.9 305.3,77.6 341.5,61.0 377.6,55.0 413.8,49.6 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,105.0 124.4,96.7 160.5,94.1 196.7,86.8 232.9,80.1 269.1,70.1 305.3,72.9 341.5,56.8 377.6,51.7 413.8,38.7 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 10.8 ns | 40.5 ns | 42.9 ns | 25.8 ns | 38.4 ns |
| D38 | 16.1 ns | 41.4 ns | 74.5 ns | 66.4 ns | 94.4 ns |
| D57 | 17.3 ns | 25.5 ns | 58.2 ns | 359 ns | 390 ns |
| D76 | 17.9 ns | 68.5 ns | 359 ns | 389 ns | 607 ns |
| D115 | 21.1 ns | 49.6 ns | 371 ns | 606 ns | 935 ns |
| D153 | 14.7 ns | 379 ns | 622 ns | 1.05 µs | 1.57 µs |
| D230 | 22.4 ns | 344 ns | 1.1 µs | 1.84 µs | 2.78 µs |
| D307 | 44 ns | 512 ns | 1.72 µs | 1.72 µs | 3.12 µs |
| D462 | 63.8 ns | 1.18 µs | 3.14 µs | 6.02 µs | 7.47 µs |
| D616 | 73.3 ns | 2.08 µs | 5.47 µs | 10.3 µs | 11.1 µs |
| D924 | 94.9 ns | 3.26 µs | 7.94 µs | 15.3 µs | 27.3 µs |
| D1232 | 100 ns | 5.59 µs | 16 µs | 20.7 µs | 36.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.4 88.2,199.7 124.4,198.1 160.5,197.4 196.7,193.8 232.9,201.7 269.1,192.5 305.3,177.8 341.5,169.8 377.6,166.7 413.8,161.1 450.0,160.0 450.0,31.8 413.8,38.2 377.6,57.6 341.5,66.3 305.3,85.3 269.1,87.8 232.9,100.2 196.7,111.5 160.5,120.8 124.4,130.4 88.2,161.3 52.0,180.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.4 88.2,199.7 124.4,198.1 160.5,197.4 196.7,193.8 232.9,201.7 269.1,192.5 305.3,177.8 341.5,169.8 377.6,166.7 413.8,161.1 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.1 124.4,189.7 160.5,168.2 196.7,175.2 232.9,131.1 269.1,133.2 305.3,124.5 341.5,106.4 377.6,94.1 413.8,84.3 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,166.4 124.4,171.7 160.5,132.2 196.7,131.5 232.9,120.3 269.1,108.0 305.3,98.2 341.5,85.2 377.6,73.1 413.8,65.0 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.4 88.2,168.9 124.4,132.2 160.5,130.5 196.7,120.9 232.9,109.0 269.1,96.8 305.3,98.2 341.5,71.0 377.6,59.4 413.8,50.7 450.0,44.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.8 88.2,161.3 124.4,130.4 160.5,120.8 196.7,111.5 232.9,100.2 269.1,87.8 305.3,85.3 341.5,66.3 377.6,57.6 413.8,38.2 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.91 ns | 281 ns | 409 ns | 265 ns | 403 ns |
| D38 | 2.49 ns | 383 ns | 422 ns | 393 ns | 398 ns |
| D57 | 268 ns | 271 ns | 414 ns | 484 ns | 607 ns |
| D76 | 273 ns | 473 ns | 485 ns | 687 ns | 627 ns |
| D115 | 287 ns | 304 ns | 641 ns | 782 ns | 881 ns |
| D153 | 183 ns | 517 ns | 658 ns | 1.13 µs | 1.32 µs |
| D230 | 461 ns | 507 ns | 1.16 µs | 1.28 µs | 1.84 µs |
| D307 | 663 ns | 473 ns | 1.22 µs | 832 ns | 5.9 µs |
| D462 | 1.3 µs | 2.79 µs | 3.3 µs | 4.32 µs | 4.97 µs |
| D616 | 1.38 µs | 1.56 µs | 1.88 µs | 2.95 µs | 3.34 µs |
| D924 | 1.88 µs | 2.08 µs | 2.17 µs | 2.53 µs | 5.13 µs |
| D1232 | 3.05 µs | 3.14 µs | 3.67 µs | 4.42 µs | 5.39 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.8 88.2,190.2 124.4,88.6 160.5,88.2 196.7,87.1 232.9,96.8 269.1,76.8 305.3,68.9 341.5,54.3 377.6,53.0 413.8,46.3 450.0,35.8 450.0,23.4 413.8,24.5 377.6,33.8 341.5,25.2 305.3,21.5 269.1,46.7 232.9,54.0 196.7,62.7 160.5,70.1 124.4,70.9 88.2,80.0 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.8 88.2,190.2 124.4,88.6 160.5,88.2 196.7,87.1 232.9,96.8 269.1,76.8 305.3,68.9 341.5,54.3 377.6,53.0 413.8,46.3 450.0,35.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,87.6 88.2,80.9 124.4,88.3 160.5,76.3 196.7,85.9 232.9,74.3 269.1,74.8 305.3,76.3 341.5,37.8 377.6,50.3 413.8,44.1 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.4 88.2,78.8 124.4,79.1 160.5,75.7 196.7,69.6 232.9,69.1 269.1,56.8 305.3,55.7 341.5,34.0 377.6,46.3 413.8,43.1 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,88.8 88.2,80.3 124.4,75.8 160.5,68.2 196.7,65.3 232.9,57.4 269.1,54.6 305.3,64.0 341.5,28.2 377.6,36.5 413.8,39.8 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,80.0 124.4,70.9 160.5,70.1 196.7,62.7 232.9,54.0 269.1,46.7 305.3,21.5 341.5,25.2 377.6,33.8 413.8,24.5 450.0,23.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.88 µs | 3.19 µs | 5.93 µs | 3.88 µs | 6.7 µs |
| D38 | 2.83 µs | 5.8 µs | 7.37 µs | 8.5 µs | 9.82 µs |
| D57 | 2.41 µs | 1.38 µs | 2.2 µs | 2.69 µs | 2.78 µs |
| D76 | 2.37 µs | 2.63 µs | 2.63 µs | 3.08 µs | 2.87 µs |
| D115 | 4.69 µs | 3.28 µs | 5.36 µs | 4.34 µs | 5.31 µs |
| D153 | 2.98 µs | 5.18 µs | 5.5 µs | 6.49 µs | 6.43 µs |
| D230 | 5.54 µs | 5.93 µs | 8.98 µs | 8.95 µs | 10 µs |
| D307 | 9.35 µs | 7.53 µs | 13.5 µs | 8.03 µs | 8.82 µs |
| D462 | 10 µs | 12.2 µs | 14.1 µs | 17.5 µs | 18.1 µs |
| D616 | 16.2 µs | 25.4 µs | 30 µs | 35.5 µs | 36.4 µs |
| D924 | 20.7 µs | 39.8 µs | 42.6 µs | 47.5 µs | 92.3 µs |
| D1232 | 32.9 µs | 62.2 µs | 78.1 µs | 99.9 µs | 107 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,179.9 124.4,184.5 160.5,185.0 196.7,165.3 232.9,178.4 269.1,160.4 305.3,145.3 341.5,143.2 377.6,129.4 413.8,122.2 450.0,108.9 450.0,74.6 413.8,79.0 377.6,105.9 341.5,126.1 305.3,147.0 269.1,143.3 232.9,156.1 196.7,161.6 160.5,179.5 124.4,180.4 88.2,143.9 52.0,154.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,179.9 124.4,184.5 160.5,185.0 196.7,165.3 232.9,178.4 269.1,160.4 305.3,145.3 341.5,143.2 377.6,129.4 413.8,122.2 450.0,108.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.5 88.2,159.1 124.4,200.6 160.5,182.0 196.7,175.6 232.9,162.4 269.1,158.5 305.3,151.6 341.5,137.7 377.6,116.4 413.8,103.4 450.0,90.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.5 88.2,152.2 124.4,187.1 160.5,182.1 196.7,161.4 232.9,160.7 269.1,146.5 305.3,134.6 341.5,133.3 377.6,111.6 413.8,101.4 450.0,83.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.8 88.2,148.0 124.4,181.4 160.5,177.5 196.7,167.5 232.9,155.8 269.1,146.5 305.3,149.7 341.5,127.2 377.6,106.6 413.8,98.2 450.0,76.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.9 88.2,143.9 124.4,180.4 160.5,179.5 196.7,161.6 232.9,156.1 269.1,143.3 305.3,147.0 341.5,126.1 377.6,105.9 413.8,79.0 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.58 ns | 2.1 µs | 4.26 µs | 2.57 µs | 4.56 µs |
| D38 | 6.54 ns | 3.88 µs | 4.9 µs | 4.95 µs | 5.4 µs |
| D57 | 56.7 ns | 2.4 µs | 3.76 µs | 4.7 µs | 5.14 µs |
| D76 | 73.6 ns | 4.14 µs | 4.71 µs | 5.66 µs | 5.6 µs |
| D115 | 137 ns | 5.45 µs | 9.04 µs | 7.95 µs | 10.1 µs |
| D153 | 108 ns | 8.68 µs | 9.75 µs | 12 µs | 12.2 µs |
| D230 | 242 ns | 10.6 µs | 16.7 µs | 18.4 µs | 21.5 µs |
| D307 | 367 ns | 13.9 µs | 23.1 µs | 16.5 µs | 18.7 µs |
| D462 | 685 ns | 69.7 µs | 138 µs | 222 µs | 262 µs |
| D616 | 750 ns | 192 µs | 330 µs | 353 µs | 446 µs |
| D924 | 893 ns | 416 µs | 333 µs | 548 µs | 1.74 ms |
| D1232 | 1.37 µs | 774 µs | 673 µs | 1.8 ms | 2.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.7 88.2,186.7 124.4,159.9 160.5,156.7 196.7,148.9 232.9,151.9 269.1,141.9 305.3,136.7 341.5,129.0 377.6,127.9 413.8,125.7 450.0,120.4 450.0,29.2 413.8,31.7 377.6,48.6 341.5,55.2 305.3,87.9 269.1,86.2 232.9,93.2 196.7,95.6 160.5,102.9 124.4,104.0 88.2,103.4 52.0,105.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.7 88.2,186.7 124.4,159.9 160.5,156.7 196.7,148.9 232.9,151.9 269.1,141.9 305.3,136.7 341.5,129.0 377.6,127.9 413.8,125.7 450.0,120.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,107.5 124.4,113.4 160.5,106.6 196.7,103.3 232.9,97.5 269.1,95.0 305.3,91.6 341.5,71.6 377.6,59.1 413.8,49.4 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,104.6 124.4,107.9 160.5,105.1 196.7,97.0 232.9,96.0 269.1,89.3 305.3,85.4 341.5,63.2 377.6,52.3 413.8,52.2 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,104.5 124.4,105.1 160.5,102.8 196.7,98.6 232.9,93.4 269.1,88.2 305.3,89.5 341.5,57.3 377.6,51.5 413.8,46.0 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.4 88.2,103.4 124.4,104.0 160.5,102.9 196.7,95.6 232.9,93.2 269.1,86.2 305.3,87.9 341.5,55.2 377.6,48.6 413.8,31.7 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.89 ns | 13.3 ns | 18.5 ns | 21.3 ns | 28 ns |
| D38 | 7.12 ns | 20.1 ns | 32.7 ns | 416 ns | 606 ns |
| D57 | 159 ns | 154 ns | 344 ns | 766 ns | 762 ns |
| D76 | 192 ns | 268 ns | 779 ns | 722 ns | 1.22 µs |
| D115 | 86 ns | 343 ns | 997 ns | 987 ns | 1.38 µs |
| D153 | 68 ns | 1.1 µs | 1.59 µs | 1.84 µs | 2.65 µs |
| D230 | 130 ns | 351 ns | 2.39 µs | 3.4 µs | 4.17 µs |
| D307 | 137 ns | 1.54 µs | 3.65 µs | 2.7 µs | 4.12 µs |
| D462 | 185 ns | 3.7 µs | 6.02 µs | 9.7 µs | 12.1 µs |
| D616 | 232 ns | 1.91 µs | 10.7 µs | 14.3 µs | 16.3 µs |
| D924 | 229 ns | 2.69 µs | 11.9 µs | 18.5 µs | 35.3 µs |
| D1232 | 326 ns | 15.8 µs | 22.5 µs | 32 µs | 44.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.4 88.2,175.9 124.4,121.9 160.5,118.6 196.7,132.6 232.9,136.7 269.1,125.4 305.3,124.5 341.5,119.3 377.6,115.4 413.8,115.6 450.0,109.4 450.0,24.2 413.8,28.1 377.6,41.6 341.5,46.8 305.3,65.4 269.1,65.2 232.9,73.1 196.7,84.5 160.5,86.6 124.4,94.7 88.2,98.7 52.0,152.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.4 88.2,175.9 124.4,121.9 160.5,118.6 196.7,132.6 232.9,136.7 269.1,125.4 305.3,124.5 341.5,119.3 377.6,115.4 413.8,115.6 450.0,109.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,157.9 124.4,122.5 160.5,112.9 196.7,108.6 232.9,88.3 269.1,108.2 305.3,82.5 341.5,67.3 377.6,78.8 413.8,72.8 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,149.4 124.4,108.5 160.5,94.3 196.7,90.0 232.9,81.9 269.1,74.8 305.3,67.5 341.5,58.8 377.6,48.8 413.8,47.0 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.8 88.2,105.2 124.4,94.6 160.5,95.7 196.7,90.2 232.9,79.4 269.1,68.7 305.3,72.7 341.5,50.5 377.6,43.7 413.8,39.4 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.1 88.2,98.7 124.4,94.7 160.5,86.6 196.7,84.5 232.9,73.1 269.1,65.2 305.3,65.4 341.5,46.8 377.6,41.6 413.8,28.1 450.0,24.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
