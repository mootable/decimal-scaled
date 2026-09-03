# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 123 ns | 132 ns | 154 ns | 305 ns | 227 ns |
| D38 | 236 ns | 280 ns | 462 ns | 430 ns | 835 ns |
| D57 | 326 ns | 405 ns | 698 ns | 1.13 µs | 1.13 µs |
| D76 | 518 ns | 746 ns | 888 ns | 1.03 µs | 1.91 µs |
| D115 | 193 ns | 742 ns | 2.36 µs | 3.08 µs | 4.27 µs |
| D153 | 222 ns | 831 ns | 3.5 µs | 5.56 µs | 5.36 µs |
| D230 | 211 ns | 824 ns | 6.24 µs | 9.79 µs | 13.7 µs |
| D307 | 317 ns | 1.73 µs | 11.3 µs | 17.6 µs | 20.8 µs |
| D462 | 360 ns | 2.13 µs | 20.9 µs | 35.5 µs | 29.7 µs |
| D616 | 433 ns | 4.53 µs | 40.1 µs | 49.8 µs | 60.9 µs |
| D924 | 496 ns | 9.05 µs | 69.8 µs | 132 µs | 180 µs |
| D1232 | 864 ns | 16.4 µs | 151 µs | 227 µs | 313 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.6 88.2,191.4 124.4,184.3 160.5,174.3 196.7,195.7 232.9,192.7 269.1,193.8 305.3,184.9 341.5,182.2 377.6,178.2 413.8,175.2 450.0,163.2 450.0,35.2 413.8,47.2 377.6,70.8 341.5,86.4 305.3,94.1 269.1,103.2 232.9,123.5 196.7,128.5 160.5,145.9 124.4,157.3 88.2,163.9 52.0,192.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.6 88.2,191.4 124.4,184.3 160.5,174.3 196.7,195.7 232.9,192.7 269.1,193.8 305.3,184.9 341.5,182.2 377.6,178.2 413.8,175.2 450.0,163.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,203.9 88.2,187.6 124.4,179.6 160.5,166.4 196.7,166.5 232.9,164.0 269.1,164.2 305.3,148.1 341.5,143.6 377.6,127.2 413.8,112.2 450.0,99.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.6 88.2,176.7 124.4,167.8 160.5,162.6 196.7,141.4 232.9,132.8 269.1,120.3 305.3,107.4 341.5,94.0 377.6,79.8 413.8,67.8 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.8 88.2,178.3 124.4,157.3 160.5,159.3 196.7,135.5 232.9,122.7 269.1,110.5 305.3,97.7 341.5,82.5 377.6,75.1 413.8,53.9 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.2 88.2,163.9 124.4,157.3 160.5,145.9 196.7,128.5 232.9,123.5 269.1,103.2 305.3,94.1 341.5,86.4 377.6,70.8 413.8,47.2 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 1.67 µs | 3.26 µs | 3.8 µs | 2.99 µs |
| D38 | 1.08 ns | 3.55 µs | 4.21 µs | 3.67 µs | 4.73 µs |
| D57 | 2.18 ns | 3.22 µs | 4.4 µs | 5.69 µs | 9.22 µs |
| D76 | 3.22 ns | 6.39 µs | 6.07 µs | 7.93 µs | 11.4 µs |
| D115 | 13.2 ns | 6.59 µs | 13.3 µs | 16.3 µs | 19.7 µs |
| D153 | 16.2 ns | 7.39 µs | 15.4 µs | 22.2 µs | 30.8 µs |
| D230 | 27.1 ns | 8.12 µs | 19.4 µs | 42.2 µs | 77.8 µs |
| D307 | 60.5 ns | 16.4 µs | 35.4 µs | 83.7 µs | 116 µs |
| D462 | 105 ns | 16.6 µs | 67.7 µs | 164 µs | 154 µs |
| D616 | 116 ns | 34.5 µs | 139 µs | 228 µs | 342 µs |
| D924 | 187 ns | 80.2 µs | 225 µs | 600 µs | 914 µs |
| D1232 | 368 ns | 140 µs | 445 µs | 913 µs | 2.99 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,209.1 124.4,200.3 160.5,195.5 196.7,177.9 232.9,175.4 269.1,169.0 305.3,159.1 341.5,152.3 377.6,151.0 413.8,145.1 450.0,136.7 450.0,25.0 413.8,39.7 377.6,51.9 341.5,61.8 305.3,65.3 269.1,70.3 232.9,81.8 196.7,87.3 160.5,94.0 124.4,96.7 88.2,105.0 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,209.1 124.4,200.3 160.5,195.5 196.7,177.9 232.9,175.4 269.1,169.0 305.3,159.1 341.5,152.3 377.6,151.0 413.8,145.1 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.9 88.2,108.6 124.4,109.8 160.5,101.3 196.7,100.9 232.9,99.5 269.1,98.3 305.3,89.5 341.5,89.4 377.6,80.4 413.8,69.9 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,106.4 124.4,105.9 160.5,101.9 196.7,92.2 232.9,90.4 269.1,87.5 305.3,80.0 341.5,72.0 377.6,63.1 413.8,57.1 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,108.2 124.4,102.7 160.5,98.6 196.7,89.6 232.9,85.8 269.1,77.9 305.3,69.3 341.5,61.0 377.6,56.9 413.8,44.9 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,105.0 124.4,96.7 160.5,94.0 196.7,87.3 232.9,81.8 269.1,70.3 305.3,65.3 341.5,61.8 377.6,51.9 413.8,39.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 13.4 ns | 40.5 ns | 40.8 ns | 40.3 ns | 36 ns |
| D38 | 13 ns | 43 ns | 74.5 ns | 58.4 ns | 94.3 ns |
| D57 | 17.3 ns | 32.2 ns | 67.5 ns | 363 ns | 385 ns |
| D76 | 17.1 ns | 73.8 ns | 342 ns | 315 ns | 608 ns |
| D115 | 19.9 ns | 73.1 ns | 371 ns | 707 ns | 767 ns |
| D153 | 25.4 ns | 380 ns | 624 ns | 1e+03 ns | 1.29 µs |
| D230 | 15.9 ns | 266 ns | 956 ns | 1.84 µs | 2.78 µs |
| D307 | 25.8 ns | 721 ns | 1.65 µs | 2.91 µs | 4.93 µs |
| D462 | 52.6 ns | 743 ns | 3.02 µs | 6.07 µs | 5.3 µs |
| D616 | 69.8 ns | 1.96 µs | 5.82 µs | 8.49 µs | 11.8 µs |
| D924 | 94 ns | 3.26 µs | 8.56 µs | 24.4 µs | 25.9 µs |
| D1232 | 110 ns | 6.03 µs | 20.5 µs | 24.9 µs | 47.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.7 88.2,204.3 124.4,198.1 160.5,198.4 196.7,195.1 232.9,189.7 269.1,199.9 305.3,189.4 341.5,174.0 377.6,167.8 413.8,161.3 450.0,158.0 450.0,26.3 413.8,39.4 377.6,56.4 341.5,73.8 305.3,75.4 269.1,87.8 232.9,104.5 196.7,115.8 160.5,120.8 124.4,130.7 88.2,161.3 52.0,182.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.7 88.2,204.3 124.4,198.1 160.5,198.4 196.7,195.1 232.9,189.7 269.1,199.9 305.3,189.4 341.5,174.0 377.6,167.8 413.8,161.3 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,178.3 124.4,184.6 160.5,166.6 196.7,166.8 232.9,131.0 269.1,138.8 305.3,117.1 341.5,116.5 377.6,95.4 413.8,84.4 450.0,71.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,166.4 124.4,168.5 160.5,133.3 196.7,131.5 232.9,120.2 269.1,111.0 305.3,99.2 341.5,86.0 377.6,71.8 413.8,63.4 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,171.7 124.4,132.0 160.5,135.1 196.7,117.5 232.9,110.0 269.1,96.8 305.3,86.8 341.5,70.8 377.6,63.6 413.8,40.7 450.0,40.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.2 88.2,161.3 124.4,130.7 160.5,120.8 196.7,115.8 232.9,104.5 269.1,87.8 305.3,75.4 341.5,73.8 377.6,56.4 413.8,39.4 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.07 ns | 277 ns | 389 ns | 418 ns | 357 ns |
| D38 | 2.49 ns | 408 ns | 422 ns | 328 ns | 398 ns |
| D57 | 269 ns | 403 ns | 484 ns | 485 ns | 611 ns |
| D76 | 292 ns | 504 ns | 421 ns | 536 ns | 619 ns |
| D115 | 308 ns | 494 ns | 633 ns | 899 ns | 943 ns |
| D153 | 311 ns | 521 ns | 670 ns | 1 µs | 1.25 µs |
| D230 | 327 ns | 371 ns | 918 ns | 1.28 µs | 1.8 µs |
| D307 | 524 ns | 785 ns | 1.12 µs | 1.5 µs | 10.5 µs |
| D462 | 941 ns | 1.88 µs | 3.02 µs | 4.33 µs | 3.29 µs |
| D616 | 1.4 µs | 1.48 µs | 2.03 µs | 2.42 µs | 3.01 µs |
| D924 | 1.98 µs | 2.06 µs | 2.53 µs | 3.91 µs | 4.69 µs |
| D1232 | 3.22 µs | 3.35 µs | 4.51 µs | 5.08 µs | 6.88 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.5 88.2,194.1 124.4,112.8 160.5,111.4 196.7,110.5 232.9,110.3 269.1,109.4 305.3,101.2 341.5,91.1 377.6,84.2 413.8,78.1 450.0,69.7 450.0,56.5 413.8,63.2 377.6,70.8 341.5,69.3 305.3,49.2 269.1,79.7 232.9,86.1 196.7,91.0 160.5,98.3 124.4,98.6 88.2,106.0 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.5 88.2,194.1 124.4,112.8 160.5,111.4 196.7,110.5 232.9,110.3 269.1,109.4 305.3,101.2 341.5,91.1 377.6,84.2 413.8,78.1 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.3 88.2,105.6 124.4,105.8 160.5,101.9 196.7,102.3 232.9,101.3 269.1,107.2 305.3,94.2 341.5,79.0 377.6,83.2 413.8,77.4 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.4 88.2,105.0 124.4,102.6 160.5,105.0 196.7,98.0 232.9,97.0 269.1,91.5 305.3,88.0 341.5,70.8 377.6,77.7 413.8,73.9 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.2 88.2,109.3 124.4,102.6 160.5,100.8 196.7,91.9 232.9,89.9 269.1,85.7 305.3,82.9 341.5,64.5 377.6,74.7 413.8,66.3 450.0,61.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,106.0 124.4,98.6 160.5,98.3 196.7,91.0 232.9,86.1 269.1,79.7 305.3,49.2 341.5,69.3 377.6,70.8 413.8,63.2 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.24 µs | 3.19 µs | 5.79 µs | 6.44 µs | 5.75 µs |
| D38 | 2.52 µs | 5.92 µs | 7.37 µs | 6.71 µs | 9.68 µs |
| D57 | 2.39 µs | 2.07 µs | 2.6 µs | 2.66 µs | 2.82 µs |
| D76 | 2.68 µs | 2.81 µs | 2.15 µs | 2.4 µs | 2.9 µs |
| D115 | 5.01 µs | 5.06 µs | 5.36 µs | 5.09 µs | 5.46 µs |
| D153 | 4.73 µs | 5.21 µs | 5.54 µs | 6.03 µs | 5.81 µs |
| D230 | 4.17 µs | 4.54 µs | 6.97 µs | 8.95 µs | 10.1 µs |
| D307 | 7.64 µs | 11.5 µs | 12.5 µs | 15.1 µs | 15.4 µs |
| D462 | 6.38 µs | 8.62 µs | 11.8 µs | 17.3 µs | 11.3 µs |
| D616 | 16.2 µs | 23.7 µs | 32.7 µs | 31.2 µs | 34.5 µs |
| D924 | 23.9 µs | 39.8 µs | 47.6 µs | 76.8 µs | 85.6 µs |
| D1232 | 34.4 µs | 67.1 µs | 96.5 µs | 116 µs | 141 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.7 88.2,183.2 124.4,184.8 160.5,181.5 196.7,163.3 232.9,165.0 269.1,168.7 305.3,151.1 341.5,156.4 377.6,129.4 413.8,118.1 450.0,107.6 450.0,66.6 413.8,81.2 377.6,107.5 341.5,139.7 305.3,130.8 269.1,143.1 232.9,159.0 196.7,160.9 160.5,179.1 124.4,179.9 88.2,144.3 52.0,159.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.7 88.2,183.2 124.4,184.8 160.5,181.5 196.7,163.3 232.9,165.0 269.1,168.7 305.3,151.1 341.5,156.4 377.6,129.4 413.8,118.1 450.0,107.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.4 88.2,158.5 124.4,189.0 160.5,180.1 196.7,163.1 232.9,162.2 269.1,166.2 305.3,139.4 341.5,147.6 377.6,118.4 413.8,103.4 450.0,88.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.2 88.2,152.2 124.4,182.3 160.5,187.8 196.7,161.4 232.9,160.4 269.1,153.8 305.3,137.0 341.5,138.5 377.6,109.0 413.8,98.2 450.0,77.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.1 88.2,154.9 124.4,181.6 160.5,184.6 196.7,162.9 232.9,158.0 269.1,146.6 305.3,131.5 341.5,127.4 377.6,110.4 413.8,84.3 450.0,72.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,144.3 124.4,179.9 160.5,179.1 196.7,160.9 232.9,159.0 269.1,143.1 305.3,130.8 341.5,139.7 377.6,107.5 413.8,81.2 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.09 ns | 2.1 µs | 3.9 µs | 4.49 µs | 3.6 µs |
| D38 | 5.43 ns | 4.25 µs | 4.9 µs | 4.21 µs | 5.39 µs |
| D57 | 56.4 ns | 3.58 µs | 4.57 µs | 4.75 µs | 5.17 µs |
| D76 | 81 ns | 4.58 µs | 3.91 µs | 4.38 µs | 5.65 µs |
| D115 | 147 ns | 8.24 µs | 9.03 µs | 9.3 µs | 9.79 µs |
| D153 | 176 ns | 8.64 µs | 9.69 µs | 11.1 µs | 10.6 µs |
| D230 | 176 ns | 7.88 µs | 13 µs | 18.4 µs | 21.4 µs |
| D307 | 303 ns | 20.6 µs | 21.7 µs | 30.6 µs | 32.2 µs |
| D462 | 549 ns | 46.1 µs | 119 µs | 222 µs | 174 µs |
| D616 | 736 ns | 177 µs | 352 µs | 301 µs | 438 µs |
| D924 | 959 ns | 417 µs | 380 µs | 849 µs | 1.61 ms |
| D1232 | 1.5 µs | 832 µs | 861 µs | 2.14 ms | 2.75 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,189.0 124.4,160.0 160.5,155.5 196.7,148.1 232.9,145.9 269.1,145.8 305.3,139.1 341.5,131.7 377.6,128.1 413.8,124.8 450.0,119.3 450.0,26.0 413.8,32.7 377.6,48.8 341.5,60.3 305.3,81.2 269.1,86.3 232.9,94.9 196.7,96.0 160.5,102.8 124.4,103.9 88.2,103.4 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,189.0 124.4,160.0 160.5,155.5 196.7,148.1 232.9,145.9 269.1,145.8 305.3,139.1 341.5,131.7 377.6,128.1 413.8,124.8 450.0,119.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,106.3 124.4,108.5 160.5,105.4 196.7,98.1 232.9,97.5 269.1,98.7 305.3,86.8 341.5,76.8 377.6,60.0 413.8,49.4 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,104.6 124.4,105.4 160.5,107.4 196.7,97.0 232.9,96.1 269.1,92.5 305.3,86.1 341.5,65.0 377.6,51.5 413.8,50.6 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.7 88.2,106.5 124.4,105.0 160.5,106.0 196.7,96.6 232.9,94.5 269.1,88.1 305.3,81.8 341.5,57.3 377.6,53.5 413.8,40.6 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,103.4 124.4,103.9 160.5,102.8 196.7,96.0 232.9,94.9 269.1,86.3 305.3,81.2 341.5,60.3 377.6,48.8 413.8,32.7 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.79 ns | 13.3 ns | 17.9 ns | 29.7 ns | 28.6 ns |
| D38 | 5.44 ns | 20.3 ns | 32.7 ns | 276 ns | 602 ns |
| D57 | 160 ns | 197 ns | 486 ns | 769 ns | 763 ns |
| D76 | 209 ns | 276 ns | 516 ns | 570 ns | 1.22 µs |
| D115 | 88.4 ns | 650 ns | 992 ns | 1.15 µs | 1.39 µs |
| D153 | 96.4 ns | 1.1 µs | 1.61 µs | 1.87 µs | 2.01 µs |
| D230 | 90.3 ns | 268 ns | 1.78 µs | 3.41 µs | 4.21 µs |
| D307 | 164 ns | 2.28 µs | 3.65 µs | 4.77 µs | 7.03 µs |
| D462 | 166 ns | 2.32 µs | 5.14 µs | 9.92 µs | 7.56 µs |
| D616 | 240 ns | 1.81 µs | 11 µs | 11.2 µs | 16.4 µs |
| D924 | 234 ns | 2.68 µs | 13.6 µs | 26.5 µs | 34.5 µs |
| D1232 | 353 ns | 16.6 µs | 28.6 µs | 39.7 µs | 60.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.8 88.2,180.6 124.4,121.9 160.5,117.2 196.7,132.1 232.9,130.6 269.1,131.8 305.3,121.4 341.5,121.2 377.6,114.8 413.8,115.2 450.0,108.1 450.0,18.8 413.8,28.5 377.6,41.4 341.5,54.9 305.3,56.1 269.1,65.0 232.9,77.9 196.7,84.3 160.5,86.6 124.4,94.7 88.2,98.8 52.0,151.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.8 88.2,180.6 124.4,121.9 160.5,117.2 196.7,132.1 232.9,130.6 269.1,131.8 305.3,121.4 341.5,121.2 377.6,114.8 413.8,115.2 450.0,108.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,157.7 124.4,118.2 160.5,112.4 196.7,97.5 232.9,88.4 269.1,112.8 305.3,75.7 341.5,75.4 377.6,79.6 413.8,72.8 450.0,41.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.9 88.2,149.4 124.4,102.5 160.5,101.5 196.7,90.1 232.9,81.7 269.1,79.9 305.3,67.5 341.5,61.6 377.6,48.3 413.8,44.7 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,112.4 124.4,94.6 160.5,99.8 196.7,87.5 232.9,79.1 269.1,68.7 305.3,62.8 341.5,50.1 377.6,48.0 413.8,33.1 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,98.8 124.4,94.7 160.5,86.6 196.7,84.3 232.9,77.9 269.1,65.0 305.3,56.1 341.5,54.9 377.6,41.4 413.8,28.5 450.0,18.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
