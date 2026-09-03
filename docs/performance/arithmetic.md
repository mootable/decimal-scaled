# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.987 ns | 0.94 ns | 0.935 ns | 0.938 ns | 1.01 ns |
| D38 | 1.83 ns | 1.61 ns | 1.83 ns | 1.62 ns | 1.03 ns |
| D57 | 2.25 ns | 2.26 ns | 2.25 ns | 2.5 ns | 1.23 ns |
| D76 | 3.49 ns | 3.49 ns | 3.48 ns | 3.09 ns | 1.56 ns |
| D115 | 3.87 ns | 4.42 ns | 4.4 ns | 3.33 ns | 4.99 ns |
| D153 | 3.84 ns | 4.49 ns | 5.89 ns | 3.48 ns | 6.62 ns |
| D230 | 15.3 ns | 15.4 ns | 11.9 ns | 10 ns | 15.4 ns |
| D307 | 18.6 ns | 18.5 ns | 19.5 ns | 18.5 ns | 19.5 ns |
| D462 | 29.4 ns | 29 ns | 30.3 ns | 33.1 ns | 25.8 ns |
| D616 | 45 ns | 45.3 ns | 52.2 ns | 45.1 ns | 45.1 ns |
| D924 | 63.3 ns | 71.1 ns | 74.7 ns | 84.8 ns | 98.1 ns |
| D1232 | 77.4 ns | 73.1 ns | 107 ns | 104 ns | 87 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.3 88.2,146.9 124.4,142.4 160.5,132.9 196.7,130.6 232.9,130.8 269.1,100.7 305.3,96.6 341.5,86.6 377.6,77.3 413.8,69.9 450.0,65.5 450.0,63.0 413.8,60.4 377.6,77.3 341.5,89.4 305.3,95.5 269.1,100.7 232.9,118.9 196.7,125.1 160.5,150.3 124.4,155.6 88.2,159.3 52.0,159.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.3 88.2,146.9 124.4,142.4 160.5,132.9 196.7,130.6 232.9,130.8 269.1,100.7 305.3,96.6 341.5,86.6 377.6,77.3 413.8,69.9 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.3 88.2,149.6 124.4,142.3 160.5,132.8 196.7,127.7 232.9,127.4 269.1,100.6 305.3,96.6 341.5,86.9 377.6,77.2 413.8,67.4 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,132.9 196.7,127.8 232.9,121.5 269.1,106.3 305.3,95.5 341.5,85.9 377.6,74.1 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.5 124.4,140.1 160.5,135.5 196.7,133.9 232.9,132.9 269.1,110.0 305.3,96.6 341.5,84.0 377.6,77.3 413.8,63.6 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,159.3 124.4,155.6 160.5,150.3 196.7,125.1 232.9,118.9 269.1,100.7 305.3,95.5 341.5,89.4 377.6,77.3 413.8,60.4 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.9 ns | 5.42 ns | 5.6 ns | 7.7 ns | 7.44 ns |
| D38 | 12.7 ns | 11.6 ns | 14.3 ns | 58.6 ns | 58.7 ns |
| D57 | 23.4 ns | 33.3 ns | 67.8 ns | 121 ns | 74.6 ns |
| D76 | 25.8 ns | 65.5 ns | 84 ns | 115 ns | 91.1 ns |
| D115 | 37.3 ns | 83 ns | 104 ns | 185 ns | 253 ns |
| D153 | 41.6 ns | 103 ns | 144 ns | 164 ns | 354 ns |
| D230 | 93.3 ns | 169 ns | 169 ns | 259 ns | 630 ns |
| D307 | 116 ns | 221 ns | 398 ns | 594 ns | 974 ns |
| D462 | 188 ns | 419 ns | 566 ns | 1.17 µs | 987 ns |
| D616 | 245 ns | 601 ns | 903 ns | 1.82 µs | 2.15 µs |
| D924 | 220 ns | 984 ns | 2.07 µs | 2.86 µs | 4.69 µs |
| D1232 | 387 ns | 1.36 µs | 3.83 µs | 4.05 µs | 6.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.5 88.2,154.7 124.4,141.5 160.5,139.5 196.7,131.4 232.9,129.0 269.1,111.5 305.3,106.8 341.5,96.3 377.6,90.6 413.8,92.9 450.0,80.6 450.0,18.1 413.8,26.5 377.6,43.3 341.5,60.3 305.3,60.6 269.1,70.0 232.9,82.6 196.7,89.8 160.5,112.0 124.4,116.4 88.2,121.5 52.0,166.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.5 88.2,154.7 124.4,141.5 160.5,139.5 196.7,131.4 232.9,129.0 269.1,111.5 305.3,106.8 341.5,96.3 377.6,90.6 413.8,92.9 450.0,80.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.3 88.2,156.7 124.4,133.9 160.5,119.2 196.7,114.0 232.9,109.4 269.1,98.6 305.3,92.7 341.5,78.9 377.6,71.1 413.8,60.3 450.0,53.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.6 88.2,152.2 124.4,118.4 160.5,113.8 196.7,109.2 232.9,102.1 269.1,98.6 305.3,80.0 341.5,72.3 377.6,62.2 413.8,44.2 450.0,30.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.7 88.2,121.6 124.4,105.9 160.5,106.9 196.7,96.6 232.9,99.3 269.1,89.3 305.3,71.3 341.5,56.7 377.6,47.0 413.8,37.2 450.0,29.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,166.4 88.2,121.5 124.4,116.4 160.5,112.0 196.7,89.8 232.9,82.6 269.1,70.0 305.3,60.6 341.5,60.3 377.6,43.3 413.8,26.5 450.0,18.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.664 ns | 2.87 ns | 3.12 ns | 4.95 ns | 3.87 ns |
| D38 | 3.94 ns | 13.9 ns | 29.4 ns | 27.4 ns | 19.1 ns |
| D57 | 6.23 ns | 21.7 ns | 38.1 ns | 76.9 ns | 46.5 ns |
| D76 | 5.65 ns | 40.2 ns | 45.9 ns | 82.7 ns | 64.5 ns |
| D115 | 10.6 ns | 45.9 ns | 87.8 ns | 180 ns | 252 ns |
| D153 | 11.7 ns | 46.3 ns | 112 ns | 158 ns | 397 ns |
| D230 | 28.4 ns | 131 ns | 258 ns | 342 ns | 1.03 µs |
| D307 | 44 ns | 167 ns | 507 ns | 1.04 µs | 1.47 µs |
| D462 | 87.8 ns | 434 ns | 899 ns | 1.87 µs | 1.57 µs |
| D616 | 92.4 ns | 673 ns | 1.5 µs | 2.74 µs | 3.91 µs |
| D924 | 94.7 ns | 1.25 µs | 3.01 µs | 5.48 µs | 8.33 µs |
| D1232 | 148 ns | 1.58 µs | 5.11 µs | 8.12 µs | 11.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.6 88.2,156.8 124.4,150.2 160.5,151.6 196.7,142.5 232.9,141.1 269.1,128.2 305.3,121.9 341.5,111.9 377.6,111.1 413.8,110.8 450.0,104.3 450.0,41.2 413.8,46.0 377.6,56.9 341.5,70.1 305.3,71.1 269.1,76.2 232.9,90.0 196.7,96.6 160.5,116.4 124.4,121.1 88.2,133.9 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.6 88.2,156.8 124.4,150.2 160.5,151.6 196.7,142.5 232.9,141.1 269.1,128.2 305.3,121.9 341.5,111.9 377.6,111.1 413.8,110.8 450.0,104.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,138.6 124.4,132.1 160.5,123.2 196.7,121.3 232.9,121.2 269.1,106.1 305.3,102.5 341.5,88.7 377.6,82.4 413.8,73.4 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,127.7 124.4,124.0 160.5,121.3 196.7,111.9 232.9,108.4 269.1,96.3 305.3,86.5 341.5,78.2 377.6,70.8 413.8,60.7 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,128.7 124.4,113.8 160.5,112.8 196.7,101.5 232.9,103.4 269.1,92.2 305.3,76.2 341.5,67.6 377.6,62.1 413.8,52.1 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,133.9 124.4,121.1 160.5,116.4 196.7,96.6 232.9,90.0 269.1,76.2 305.3,71.1 341.5,70.1 377.6,56.9 413.8,46.0 450.0,41.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.508 ns | 0.623 ns | 0.623 ns | 0.622 ns | 0.482 ns |
| D38 | 1.45 ns | 1.32 ns | 1.45 ns | 1.32 ns | 0.996 ns |
| D57 | 1.87 ns | 1.87 ns | 1.87 ns | 1.74 ns | 1.08 ns |
| D76 | 2.16 ns | 2.16 ns | 2.17 ns | 2.19 ns | 1.61 ns |
| D115 | 2.46 ns | 2.83 ns | 2.82 ns | 2.78 ns | 3.55 ns |
| D153 | 2.76 ns | 2.91 ns | 4.29 ns | 2.73 ns | 4.6 ns |
| D230 | 6.65 ns | 6.65 ns | 4.53 ns | 4.05 ns | 7.44 ns |
| D307 | 10.9 ns | 11.1 ns | 12.4 ns | 11.1 ns | 12.4 ns |
| D462 | 15.1 ns | 15.3 ns | 14.4 ns | 17 ns | 12.3 ns |
| D616 | 19 ns | 20.2 ns | 24.6 ns | 19.9 ns | 19.9 ns |
| D924 | 30.9 ns | 69.4 ns | 75.7 ns | 84.8 ns | 96 ns |
| D1232 | 37.7 ns | 42.3 ns | 69.8 ns | 64.7 ns | 61.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,162.9 88.2,132.6 124.4,125.2 160.5,121.0 196.7,117.3 232.9,113.9 269.1,88.5 305.3,74.2 341.5,64.8 377.6,58.1 413.8,44.0 450.0,38.2 450.0,24.1 413.8,11.2 377.6,56.7 341.5,70.6 305.3,70.6 269.1,85.2 232.9,99.1 196.7,106.6 160.5,129.5 124.4,141.1 88.2,143.4 52.0,164.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,162.9 88.2,132.6 124.4,125.2 160.5,121.0 196.7,117.3 232.9,113.9 269.1,88.5 305.3,74.2 341.5,64.8 377.6,58.1 413.8,44.0 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,121.0 196.7,113.2 232.9,112.4 269.1,88.5 305.3,73.7 341.5,64.3 377.6,56.4 413.8,20.6 450.0,34.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,132.7 124.4,125.2 160.5,121.0 196.7,113.3 232.9,101.1 269.1,99.6 305.3,70.5 341.5,66.1 377.6,50.6 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,120.7 196.7,113.7 232.9,114.2 269.1,102.8 305.3,73.7 341.5,61.3 377.6,56.8 413.8,14.8 450.0,22.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.5 88.2,143.4 124.4,141.1 160.5,129.5 196.7,106.6 232.9,99.1 269.1,85.2 305.3,70.6 341.5,70.6 377.6,56.7 413.8,11.2 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.69 ns | 1.87 ns | 1.87 ns | 2.18 ns | 2.88 ns |
| D38 | 3.54 ns | 3.24 ns | 3.54 ns | 3.16 ns | 2.2 ns |
| D57 | 7.17 ns | 7.17 ns | 7.16 ns | 8.08 ns | 3.53 ns |
| D76 | 9.53 ns | 9.51 ns | 9.83 ns | 8.71 ns | 4.39 ns |
| D115 | 11.2 ns | 12.8 ns | 12.8 ns | 9.38 ns | 14.4 ns |
| D153 | 10.6 ns | 12.4 ns | 16 ns | 8.93 ns | 20 ns |
| D230 | 36.5 ns | 36.3 ns | 19.8 ns | 16.6 ns | 36.7 ns |
| D307 | 41.4 ns | 40.3 ns | 48.4 ns | 42.6 ns | 47.8 ns |
| D462 | 74.7 ns | 84.1 ns | 66.7 ns | 84.7 ns | 68.7 ns |
| D616 | 83.9 ns | 81.6 ns | 87.3 ns | 82 ns | 77.9 ns |
| D924 | 65.5 ns | 108 ns | 98.7 ns | 98.6 ns | 99.9 ns |
| D1232 | 99.7 ns | 89.9 ns | 131 ns | 125 ns | 87 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,173.4 124.4,153.0 160.5,144.7 196.7,140.1 232.9,141.7 269.1,105.8 305.3,102.2 341.5,85.1 377.6,81.8 413.8,88.9 450.0,76.7 450.0,80.7 413.8,76.7 377.6,83.9 341.5,87.5 305.3,98.0 269.1,105.7 232.9,123.2 196.7,132.7 160.5,167.2 124.4,173.5 88.2,187.2 52.0,179.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,173.4 124.4,153.0 160.5,144.7 196.7,140.1 232.9,141.7 269.1,105.8 305.3,102.2 341.5,85.1 377.6,81.8 413.8,88.9 450.0,76.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,144.8 196.7,136.3 232.9,137.2 269.1,106.0 305.3,103.0 341.5,81.7 377.6,82.5 413.8,74.4 450.0,79.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,173.4 124.4,153.0 160.5,143.8 196.7,136.3 232.9,129.7 269.1,123.5 305.3,97.7 341.5,88.4 377.6,80.6 413.8,77.0 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.7 124.4,149.5 160.5,147.3 196.7,145.2 232.9,146.6 269.1,128.6 305.3,101.4 341.5,81.5 377.6,82.4 413.8,77.1 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,187.2 124.4,173.5 160.5,167.2 196.7,132.7 232.9,123.2 269.1,105.7 305.3,98.0 341.5,87.5 377.6,83.9 413.8,76.7 450.0,80.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.981 ns | 0.935 ns | 0.936 ns | 0.938 ns | 1.01 ns |
| D38 | 1.81 ns | 1.61 ns | 1.81 ns | 1.6 ns | 0.947 ns |
| D57 | 2.28 ns | 2.27 ns | 2.27 ns | 2.5 ns | 1.3 ns |
| D76 | 3.45 ns | 3.45 ns | 3.45 ns | 3.09 ns | 1.82 ns |
| D115 | 4.31 ns | 4.89 ns | 4.88 ns | 4.03 ns | 5.55 ns |
| D153 | 5.21 ns | 6.4 ns | 7.57 ns | 4.67 ns | 8.46 ns |
| D230 | 17.7 ns | 17.6 ns | 13.1 ns | 10.9 ns | 17.7 ns |
| D307 | 23.6 ns | 23.4 ns | 25.2 ns | 23.4 ns | 25.1 ns |
| D462 | 38.3 ns | 37.2 ns | 36 ns | 42.8 ns | 30.8 ns |
| D616 | 46 ns | 46.2 ns | 53.3 ns | 45.7 ns | 45.7 ns |
| D924 | 67.7 ns | 71.1 ns | 75 ns | 84.8 ns | 98.1 ns |
| D1232 | 87.5 ns | 81 ns | 106 ns | 105 ns | 97.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.4 88.2,147.1 124.4,142.1 160.5,133.1 196.7,128.3 232.9,124.1 269.1,97.6 305.3,91.4 341.5,80.9 377.6,76.9 413.8,68.5 450.0,62.9 450.0,60.6 413.8,60.4 377.6,77.0 341.5,85.6 305.3,90.0 269.1,97.7 232.9,113.6 196.7,122.8 160.5,147.0 124.4,154.3 88.2,161.2 52.0,159.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.4 88.2,147.1 124.4,142.1 160.5,133.1 196.7,128.3 232.9,124.1 269.1,97.6 305.3,91.4 341.5,80.9 377.6,76.9 413.8,68.5 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.2 160.5,133.1 196.7,125.5 232.9,119.7 269.1,97.7 305.3,91.5 341.5,81.5 377.6,76.8 413.8,67.4 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.1 124.4,142.2 160.5,133.1 196.7,125.6 232.9,116.0 269.1,104.2 305.3,90.0 341.5,82.2 377.6,73.7 413.8,66.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.8 124.4,140.1 160.5,135.5 196.7,129.7 232.9,126.5 269.1,108.0 305.3,91.5 341.5,78.4 377.6,77.0 413.8,63.6 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,161.2 124.4,154.3 160.5,147.0 196.7,122.8 232.9,113.6 269.1,97.7 305.3,90.0 341.5,85.6 377.6,77.0 413.8,60.4 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
