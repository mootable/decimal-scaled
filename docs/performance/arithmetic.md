# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.819 ns | 0.941 ns | 0.935 ns | 0.935 ns | 1.05 ns |
| D38 | 1.82 ns | 1.61 ns | 1.62 ns | 1.81 ns | 1.23 ns |
| D57 | 2.28 ns | 2.27 ns | 1.71 ns | 1.3 ns | 1.72 ns |
| D76 | 3.48 ns | 3.49 ns | 1.62 ns | 3.29 ns | 3.07 ns |
| D115 | 3.87 ns | 2.95 ns | 4.99 ns | 4.4 ns | 4.99 ns |
| D153 | 4.71 ns | 5.9 ns | 6.63 ns | 6.63 ns | 6.63 ns |
| D230 | 15.6 ns | 11.7 ns | 11.9 ns | 13.8 ns | 12.8 ns |
| D307 | 19.6 ns | 11.8 ns | 19.6 ns | 18.6 ns | 19.6 ns |
| D462 | 29.6 ns | 30.6 ns | 29 ns | 25.6 ns | 32.7 ns |
| D616 | 66.2 ns | 49.7 ns | 44.8 ns | 60.6 ns | 52.2 ns |
| D924 | 52.8 ns | 81.9 ns | 64.7 ns | 84.9 ns | 75 ns |
| D1232 | 107 ns | 83.6 ns | 95.1 ns | 84.3 ns | 95 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.3 88.2,147.0 124.4,142.1 160.5,132.9 196.7,130.6 232.9,126.3 269.1,100.4 305.3,95.4 341.5,86.5 377.6,68.9 413.8,73.9 450.0,58.4 450.0,61.1 413.8,66.2 377.6,74.1 341.5,84.3 305.3,95.4 269.1,104.6 232.9,118.9 196.7,125.1 160.5,135.6 124.4,148.2 88.2,155.5 52.0,158.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.3 88.2,147.0 124.4,142.1 160.5,132.9 196.7,130.6 232.9,126.3 269.1,100.4 305.3,95.4 341.5,86.5 377.6,68.9 413.8,73.9 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.3 88.2,149.7 124.4,142.2 160.5,132.9 196.7,136.5 232.9,121.5 269.1,106.6 305.3,106.5 341.5,85.7 377.6,75.2 413.8,64.3 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.5 124.4,148.3 160.5,149.5 196.7,125.1 232.9,118.9 269.1,106.2 305.3,95.4 341.5,86.9 377.6,77.4 413.8,69.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.1 124.4,154.3 160.5,134.1 196.7,127.8 232.9,118.9 269.1,103.1 305.3,96.5 341.5,89.6 377.6,70.9 413.8,63.5 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.9 88.2,155.5 124.4,148.2 160.5,135.6 196.7,125.1 232.9,118.9 269.1,104.6 305.3,95.4 341.5,84.3 377.6,74.1 413.8,66.2 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.99 ns | 5.9 ns | 5.51 ns | 8.3 ns | 8.89 ns |
| D38 | 12.7 ns | 11.5 ns | 15.6 ns | 67.9 ns | 55.3 ns |
| D57 | 26.1 ns | 33.2 ns | 50.3 ns | 72.1 ns | 95.3 ns |
| D76 | 25.3 ns | 65.3 ns | 43.9 ns | 126 ns | 148 ns |
| D115 | 38.1 ns | 58.5 ns | 113 ns | 188 ns | 253 ns |
| D153 | 43 ns | 110 ns | 155 ns | 265 ns | 366 ns |
| D230 | 94 ns | 113 ns | 196 ns | 359 ns | 479 ns |
| D307 | 133 ns | 130 ns | 402 ns | 584 ns | 975 ns |
| D462 | 193 ns | 316 ns | 687 ns | 978 ns | 1.48 µs |
| D616 | 249 ns | 567 ns | 1.03 µs | 1.98 µs | 2.39 µs |
| D924 | 225 ns | 998 ns | 1.52 µs | 2.86 µs | 4.05 µs |
| D1232 | 546 ns | 1.56 µs | 3.45 µs | 3.62 µs | 6.99 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.1 88.2,154.8 124.4,139.1 160.5,139.9 196.7,130.9 232.9,128.3 269.1,111.3 305.3,103.8 341.5,95.7 377.6,90.2 413.8,92.4 450.0,73.2 450.0,17.8 413.8,29.6 377.6,41.1 341.5,51.5 305.3,60.6 269.1,76.0 232.9,81.8 196.7,89.8 160.5,101.5 124.4,111.0 88.2,122.9 52.0,162.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.1 88.2,154.8 124.4,139.1 160.5,139.9 196.7,130.9 232.9,128.3 269.1,111.3 305.3,103.8 341.5,95.7 377.6,90.2 413.8,92.4 450.0,73.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.5 88.2,157.1 124.4,133.9 160.5,119.2 196.7,121.6 232.9,108.0 269.1,107.3 305.3,104.3 341.5,85.0 377.6,72.3 413.8,60.0 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.9 88.2,150.3 124.4,124.9 160.5,127.9 196.7,107.4 232.9,100.4 269.1,95.4 305.3,79.8 341.5,68.1 377.6,59.4 413.8,51.0 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.0 88.2,118.4 124.4,117.1 160.5,105.0 196.7,96.3 232.9,88.8 269.1,82.3 305.3,71.7 341.5,60.5 377.6,45.2 413.8,37.2 450.0,32.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.6 88.2,122.9 124.4,111.0 160.5,101.5 196.7,89.8 232.9,81.8 269.1,76.0 305.3,60.6 341.5,51.5 377.6,41.1 413.8,29.6 450.0,17.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.816 ns | 2.99 ns | 3.02 ns | 4.93 ns | 4.95 ns |
| D38 | 3.95 ns | 14.2 ns | 26.3 ns | 30.3 ns | 21.5 ns |
| D57 | 6.54 ns | 21.6 ns | 27.5 ns | 46.3 ns | 62.7 ns |
| D76 | 5.65 ns | 38.2 ns | 26.3 ns | 79.1 ns | 102 ns |
| D115 | 10.6 ns | 34 ns | 93.3 ns | 196 ns | 254 ns |
| D153 | 13.5 ns | 50.3 ns | 118 ns | 259 ns | 396 ns |
| D230 | 28.4 ns | 78 ns | 286 ns | 473 ns | 737 ns |
| D307 | 54.3 ns | 94.6 ns | 508 ns | 1.03 µs | 1.47 µs |
| D462 | 73.7 ns | 315 ns | 1.27 µs | 1.48 µs | 2.62 µs |
| D616 | 113 ns | 578 ns | 1.76 µs | 2.95 µs | 4.19 µs |
| D924 | 90.1 ns | 1.25 µs | 2.34 µs | 5.48 µs | 7.7 µs |
| D1232 | 197 ns | 1.86 µs | 4.71 µs | 7.09 µs | 13 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,156.8 124.4,149.5 160.5,151.6 196.7,142.5 232.9,139.0 269.1,128.2 305.3,118.8 341.5,114.4 377.6,108.2 413.8,111.5 450.0,100.2 450.0,39.5 413.8,47.1 377.6,55.9 341.5,62.7 305.3,71.1 269.1,81.1 232.9,90.1 196.7,96.5 160.5,109.7 124.4,116.8 88.2,132.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,156.8 124.4,149.5 160.5,151.6 196.7,142.5 232.9,139.0 269.1,128.2 305.3,118.8 341.5,114.4 377.6,108.2 413.8,111.5 450.0,100.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.8 88.2,138.2 124.4,132.2 160.5,123.9 196.7,125.6 232.9,119.9 269.1,113.6 305.3,110.8 341.5,93.4 377.6,84.6 413.8,73.4 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.7 88.2,129.4 124.4,128.7 160.5,129.4 196.7,111.0 232.9,107.6 269.1,94.8 305.3,86.5 341.5,73.2 377.6,68.5 413.8,64.4 450.0,54.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,127.3 124.4,121.2 160.5,113.4 196.7,100.3 232.9,96.2 269.1,87.5 305.3,76.2 341.5,71.0 377.6,61.0 413.8,52.0 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.2 124.4,116.8 160.5,109.7 196.7,96.5 232.9,90.1 269.1,81.1 305.3,71.1 341.5,62.7 377.6,55.9 413.8,47.1 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.546 ns | 0.623 ns | 0.622 ns | 0.623 ns | 0.703 ns |
| D38 | 1.45 ns | 1.32 ns | 1.32 ns | 1.45 ns | 1.11 ns |
| D57 | 1.68 ns | 1.68 ns | 1.48 ns | 1.05 ns | 1.48 ns |
| D76 | 2.16 ns | 2.16 ns | 1.39 ns | 2.06 ns | 2.49 ns |
| D115 | 2.46 ns | 2.58 ns | 3.17 ns | 3.25 ns | 3.55 ns |
| D153 | 3.38 ns | 3.82 ns | 4.59 ns | 4.6 ns | 4.6 ns |
| D230 | 6.65 ns | 4.19 ns | 5.62 ns | 5.36 ns | 5.06 ns |
| D307 | 12.3 ns | 5.49 ns | 12.4 ns | 11 ns | 12.4 ns |
| D462 | 15 ns | 14.5 ns | 14.9 ns | 11.8 ns | 16.7 ns |
| D616 | 23 ns | 17.3 ns | 19.9 ns | 21.8 ns | 21.9 ns |
| D924 | 25 ns | 71.5 ns | 57.5 ns | 85.1 ns | 76 ns |
| D1232 | 54.4 ns | 51.8 ns | 61.5 ns | 52.3 ns | 61.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.9 88.2,132.6 124.4,128.3 160.5,121.0 196.7,117.3 232.9,108.0 269.1,88.5 305.3,70.7 341.5,64.8 377.6,52.6 413.8,50.2 450.0,27.6 450.0,24.1 413.8,18.0 377.6,53.9 341.5,61.9 305.3,70.5 269.1,96.4 232.9,99.2 196.7,106.7 160.5,117.0 124.4,132.0 88.2,140.3 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.9 88.2,132.6 124.4,128.3 160.5,121.0 196.7,117.3 232.9,108.0 269.1,88.5 305.3,70.7 341.5,64.8 377.6,52.6 413.8,50.2 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.0 88.2,135.2 124.4,128.3 160.5,121.0 196.7,115.9 232.9,104.6 269.1,101.9 305.3,94.0 341.5,65.9 377.6,60.7 413.8,19.7 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,132.0 160.5,133.8 196.7,110.0 232.9,99.2 269.1,93.4 305.3,70.5 341.5,65.1 377.6,56.8 413.8,26.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,132.7 124.4,142.0 160.5,122.5 196.7,109.2 232.9,99.1 269.1,94.7 305.3,73.9 341.5,72.0 377.6,54.1 413.8,14.7 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,140.3 124.4,132.0 160.5,117.0 196.7,106.7 232.9,99.2 269.1,96.4 305.3,70.5 341.5,61.9 377.6,53.9 413.8,18.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 ns | 1.87 ns | 1.87 ns | 2.18 ns | 2.46 ns |
| D38 | 3.54 ns | 3.25 ns | 3.25 ns | 3.56 ns | 2.3 ns |
| D57 | 7.16 ns | 7.17 ns | 4.79 ns | 3.46 ns | 4.31 ns |
| D76 | 9.5 ns | 9.53 ns | 4.37 ns | 9.32 ns | 8.52 ns |
| D115 | 11.2 ns | 8.05 ns | 14.4 ns | 12.8 ns | 14.4 ns |
| D153 | 13 ns | 15.9 ns | 20 ns | 20.1 ns | 27.9 ns |
| D230 | 36.4 ns | 19.6 ns | 28.1 ns | 23 ns | 20.5 ns |
| D307 | 48 ns | 23.7 ns | 48 ns | 43.2 ns | 47.8 ns |
| D462 | 75.1 ns | 68 ns | 75 ns | 57.3 ns | 85.7 ns |
| D616 | 105 ns | 75.8 ns | 78.4 ns | 101 ns | 93 ns |
| D924 | 67.2 ns | 111 ns | 72.2 ns | 102 ns | 88.2 ns |
| D1232 | 148 ns | 115 ns | 127 ns | 103 ns | 111 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,173.4 124.4,153.0 160.5,144.8 196.7,140.1 232.9,135.7 269.1,105.9 305.3,97.9 341.5,85.0 377.6,75.3 413.8,88.2 450.0,65.4 450.0,73.6 413.8,80.3 377.6,78.8 341.5,81.1 305.3,98.0 269.1,122.6 232.9,113.6 196.7,132.8 160.5,148.0 124.4,167.7 88.2,185.9 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,173.4 124.4,153.0 160.5,144.8 196.7,140.1 232.9,135.7 269.1,105.9 305.3,97.9 341.5,85.0 377.6,75.3 413.8,88.2 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,144.7 196.7,149.6 232.9,129.9 269.1,123.9 305.3,118.3 341.5,87.8 377.6,84.7 413.8,73.7 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,175.9 124.4,164.6 160.5,167.3 196.7,132.7 232.9,123.2 269.1,113.4 305.3,97.9 341.5,85.0 377.6,83.7 413.8,86.1 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,173.2 124.4,174.1 160.5,145.4 196.7,136.3 232.9,123.2 269.1,119.2 305.3,101.0 341.5,92.8 377.6,76.4 413.8,76.1 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,185.9 124.4,167.7 160.5,148.0 196.7,132.8 232.9,113.6 269.1,122.6 305.3,98.0 341.5,81.1 377.6,78.8 413.8,80.3 450.0,73.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.818 ns | 0.935 ns | 0.935 ns | 0.935 ns | 1.06 ns |
| D38 | 1.81 ns | 1.6 ns | 1.61 ns | 1.82 ns | 1.1 ns |
| D57 | 2.25 ns | 2.25 ns | 1.93 ns | 1.3 ns | 1.88 ns |
| D76 | 3.45 ns | 3.45 ns | 1.82 ns | 3.26 ns | 3.08 ns |
| D115 | 4.3 ns | 3.77 ns | 5.54 ns | 4.85 ns | 5.53 ns |
| D153 | 6.37 ns | 7.6 ns | 8.44 ns | 8.45 ns | 8.47 ns |
| D230 | 17.7 ns | 12.9 ns | 13.7 ns | 15.2 ns | 13.6 ns |
| D307 | 25.1 ns | 13.4 ns | 25.2 ns | 23.4 ns | 25.2 ns |
| D462 | 37.5 ns | 36.1 ns | 37.2 ns | 32.1 ns | 42.4 ns |
| D616 | 58.1 ns | 50.2 ns | 45.8 ns | 62.7 ns | 55.2 ns |
| D924 | 51.6 ns | 79.4 ns | 69.3 ns | 84.9 ns | 75.3 ns |
| D1232 | 106 ns | 83.7 ns | 96.2 ns | 84.2 ns | 95.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.4 88.2,147.1 124.4,142.4 160.5,133.1 196.7,128.3 232.9,119.8 269.1,97.6 305.3,90.0 341.5,81.3 377.6,71.8 413.8,74.3 450.0,58.7 450.0,61.0 413.8,66.2 377.6,72.9 341.5,78.6 305.3,89.9 269.1,103.3 232.9,113.6 196.7,122.9 160.5,135.6 124.4,146.3 88.2,157.9 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.4 88.2,147.1 124.4,142.4 160.5,133.1 196.7,128.3 232.9,119.8 269.1,97.6 305.3,90.0 341.5,81.3 377.6,71.8 413.8,74.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.8 124.4,142.4 160.5,133.1 196.7,131.2 232.9,116.0 269.1,104.4 305.3,103.6 341.5,82.1 377.6,75.0 413.8,65.0 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.7 124.4,145.7 160.5,147.0 196.7,122.8 232.9,113.7 269.1,103.1 305.3,90.0 341.5,81.5 377.6,76.9 413.8,68.0 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,154.3 160.5,134.4 196.7,125.7 232.9,113.6 269.1,100.9 305.3,91.5 341.5,84.7 377.6,70.2 413.8,63.5 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,157.9 124.4,146.3 160.5,135.6 196.7,122.9 232.9,113.6 269.1,103.3 305.3,89.9 341.5,78.6 377.6,72.9 413.8,66.2 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
