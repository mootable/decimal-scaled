# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.864 ns | 0.92 ns | 1.05 ns | 0.938 ns |
| D38 | 0.981 ns | 1.81 ns | 2.12 ns | 0.981 ns | 1.81 ns |
| D57 | 2.25 ns | 2.49 ns | 2.49 ns | 2.49 ns | 2.49 ns |
| D76 | 3.49 ns | 1.64 ns | 3.08 ns | 3.48 ns | 1.79 ns |
| D115 | 3.87 ns | 4.4 ns | 3.33 ns | 4.41 ns | 5 ns |
| D153 | 6.62 ns | 5.9 ns | 5.9 ns | 5.9 ns | 6.62 ns |
| D230 | 15.3 ns | 15.3 ns | 13.9 ns | 13.9 ns | 14 ns |
| D307 | 13.4 ns | 15.2 ns | 18.6 ns | 19.6 ns | 18.5 ns |
| D462 | 35 ns | 29.4 ns | 28.8 ns | 22.9 ns | 33.2 ns |
| D616 | 51.1 ns | 49 ns | 59.9 ns | 57 ns | 56.2 ns |
| D924 | 75.8 ns | 74.6 ns | 85.1 ns | 79.2 ns | 84.5 ns |
| D1232 | 106 ns | 106 ns | 95.4 ns | 96.4 ns | 67.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,160.4 124.4,142.3 160.5,132.9 196.7,130.6 232.9,118.9 269.1,100.7 305.3,103.6 341.5,82.8 377.6,74.6 413.8,66.0 450.0,58.7 450.0,68.4 413.8,63.7 377.6,72.5 341.5,83.9 305.3,96.6 269.1,102.7 232.9,118.9 196.7,125.1 160.5,147.4 124.4,140.2 88.2,147.1 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,160.4 124.4,142.3 160.5,132.9 196.7,130.6 232.9,118.9 269.1,100.7 305.3,103.6 341.5,82.8 377.6,74.6 413.8,66.0 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.2 88.2,147.1 124.4,140.2 160.5,149.2 196.7,127.8 232.9,121.5 269.1,100.7 305.3,100.9 341.5,86.6 377.6,75.5 413.8,66.4 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,143.7 124.4,140.2 160.5,135.6 196.7,133.9 232.9,121.5 269.1,102.8 305.3,96.6 341.5,87.0 377.6,71.1 413.8,63.5 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,160.4 124.4,140.2 160.5,132.9 196.7,127.8 232.9,121.5 269.1,102.9 305.3,95.4 341.5,92.0 377.6,72.2 413.8,65.1 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.1 124.4,140.2 160.5,147.4 196.7,125.1 232.9,118.9 269.1,102.7 305.3,96.6 341.5,83.9 377.6,72.5 413.8,63.7 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.44 ns | 4.03 ns | 5.33 ns | 8.79 ns | 8.52 ns |
| D38 | 7.34 ns | 12.7 ns | 16.7 ns | 54.4 ns | 68.2 ns |
| D57 | 23.4 ns | 34.5 ns | 74.8 ns | 120 ns | 120 ns |
| D76 | 26.3 ns | 33.4 ns | 75.3 ns | 121 ns | 107 ns |
| D115 | 37.1 ns | 83.1 ns | 95.1 ns | 189 ns | 254 ns |
| D153 | 57.8 ns | 109 ns | 143 ns | 264 ns | 355 ns |
| D230 | 95.1 ns | 171 ns | 231 ns | 392 ns | 571 ns |
| D307 | 71.5 ns | 192 ns | 365 ns | 642 ns | 878 ns |
| D462 | 213 ns | 419 ns | 683 ns | 1.01 µs | 1.48 µs |
| D616 | 232 ns | 638 ns | 1.14 µs | 1.99 µs | 2.4 µs |
| D924 | 352 ns | 1.11 µs | 2.28 µs | 2.24 µs | 4.2 µs |
| D1232 | 564 ns | 1.95 µs | 3.48 µs | 4.14 µs | 4.14 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.5 88.2,166.7 124.4,141.6 160.5,139.0 196.7,131.5 232.9,121.9 269.1,111.1 305.3,117.3 341.5,93.6 377.6,91.7 413.8,82.7 450.0,72.4 450.0,29.1 413.8,28.8 377.6,41.0 341.5,51.4 305.3,62.8 269.1,72.2 232.9,82.5 196.7,89.8 160.5,108.6 124.4,106.0 88.2,118.3 52.0,163.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.5 88.2,166.7 124.4,141.6 160.5,139.0 196.7,131.5 232.9,121.9 269.1,111.1 305.3,117.3 341.5,93.6 377.6,91.7 413.8,82.7 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.8 88.2,154.9 124.4,133.1 160.5,133.8 196.7,114.0 232.9,108.1 269.1,98.3 305.3,95.8 341.5,78.9 377.6,69.8 413.8,57.7 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.7 88.2,148.8 124.4,116.3 160.5,116.2 196.7,111.1 232.9,102.2 269.1,91.8 305.3,81.9 341.5,68.3 377.6,57.2 413.8,42.1 450.0,32.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.8 88.2,123.2 124.4,106.0 160.5,105.9 196.7,96.1 232.9,88.9 269.1,80.3 305.3,69.6 341.5,59.8 377.6,45.0 413.8,42.5 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.5 88.2,118.3 124.4,106.0 160.5,108.6 196.7,89.8 232.9,82.5 269.1,72.2 305.3,62.8 341.5,51.4 377.6,41.0 413.8,28.8 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.75 ns | 2.76 ns | 5.02 ns | 5.05 ns |
| D38 | 2.52 ns | 13.7 ns | 41.7 ns | 18 ns | 37.5 ns |
| D57 | 6.23 ns | 21.8 ns | 35 ns | 77.4 ns | 77.3 ns |
| D76 | 5.66 ns | 21.1 ns | 42.8 ns | 84.5 ns | 72.9 ns |
| D115 | 10.5 ns | 46.5 ns | 76.6 ns | 193 ns | 254 ns |
| D153 | 16.8 ns | 59.8 ns | 112 ns | 234 ns | 396 ns |
| D230 | 28 ns | 132 ns | 333 ns | 504 ns | 993 ns |
| D307 | 25.9 ns | 143 ns | 450 ns | 1.07 µs | 1.41 µs |
| D462 | 77 ns | 415 ns | 1.24 µs | 1.65 µs | 2.62 µs |
| D616 | 93.3 ns | 639 ns | 1.87 µs | 2.92 µs | 4.19 µs |
| D924 | 135 ns | 1.52 µs | 3.22 µs | 4.28 µs | 7.69 µs |
| D1232 | 196 ns | 2.37 µs | 4.7 µs | 8.31 µs | 7.83 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.9 88.2,189.9 124.4,170.3 160.5,172.4 196.7,159.0 232.9,148.7 269.1,137.7 305.3,139.3 341.5,115.7 377.6,111.5 413.8,103.5 450.0,95.4 450.0,15.3 413.8,15.7 377.6,28.9 341.5,39.1 305.3,52.6 269.1,60.2 232.9,80.1 196.7,89.8 160.5,116.9 124.4,115.6 88.2,131.3 52.0,174.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.9 88.2,189.9 124.4,170.3 160.5,172.4 196.7,159.0 232.9,148.7 269.1,137.7 305.3,139.3 341.5,115.7 377.6,111.5 413.8,103.5 450.0,95.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.9 88.2,153.1 124.4,143.1 160.5,143.7 196.7,126.6 232.9,121.1 269.1,104.0 305.3,102.2 341.5,79.1 377.6,69.7 413.8,51.0 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.0 88.2,129.0 124.4,132.8 160.5,128.4 196.7,115.8 232.9,107.5 269.1,83.9 305.3,77.3 341.5,55.3 377.6,46.4 413.8,34.6 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.0 88.2,147.2 124.4,115.6 160.5,113.7 196.7,95.7 232.9,91.5 269.1,74.9 305.3,58.6 341.5,49.1 377.6,36.7 413.8,28.4 450.0,14.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.8 88.2,131.3 124.4,115.6 160.5,116.9 196.7,89.8 232.9,80.1 269.1,60.2 305.3,52.6 341.5,39.1 377.6,28.9 413.8,15.7 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.324 ns | 0.608 ns | 0.704 ns | 0.623 ns |
| D38 | 0.933 ns | 1.45 ns | 1.69 ns | 0.932 ns | 1.45 ns |
| D57 | 1.87 ns | 1.74 ns | 1.74 ns | 1.74 ns | 1.74 ns |
| D76 | 2.17 ns | 1.39 ns | 2.1 ns | 2.17 ns | 1.73 ns |
| D115 | 2.46 ns | 2.86 ns | 2.31 ns | 3.25 ns | 3.55 ns |
| D153 | 4.22 ns | 3.83 ns | 4.29 ns | 4.3 ns | 4.6 ns |
| D230 | 6.65 ns | 6.65 ns | 7.18 ns | 7.17 ns | 7.18 ns |
| D307 | 5.59 ns | 9.59 ns | 11 ns | 12.4 ns | 11 ns |
| D462 | 15 ns | 15.5 ns | 14.9 ns | 10.8 ns | 17 ns |
| D616 | 17.9 ns | 25 ns | 21.6 ns | 22 ns | 22 ns |
| D924 | 54.8 ns | 76.1 ns | 84.7 ns | 74.4 ns | 76.4 ns |
| D1232 | 54.4 ns | 69.7 ns | 62.2 ns | 61.4 ns | 41.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,145.3 124.4,125.2 160.5,120.9 196.7,117.3 232.9,101.7 269.1,88.5 305.3,93.5 341.5,64.9 377.6,59.8 413.8,27.4 450.0,27.6 450.0,35.3 413.8,17.8 377.6,53.9 341.5,61.3 305.3,73.9 269.1,86.3 232.9,99.2 196.7,106.6 160.5,127.5 124.4,127.3 88.2,132.6 52.0,157.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,145.3 124.4,125.2 160.5,120.9 196.7,117.3 232.9,101.7 269.1,88.5 305.3,93.5 341.5,64.9 377.6,59.8 413.8,27.4 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.0 88.2,132.6 124.4,127.3 160.5,133.9 196.7,112.9 232.9,104.5 269.1,88.5 305.3,77.9 341.5,64.0 377.6,50.1 413.8,17.9 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.7 88.2,128.2 124.4,127.3 160.5,121.9 196.7,119.1 232.9,101.1 269.1,86.3 305.3,74.0 341.5,65.1 377.6,54.4 413.8,14.8 450.0,23.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,145.4 124.4,127.4 160.5,120.9 196.7,109.2 232.9,101.1 269.1,86.3 305.3,70.5 341.5,74.4 377.6,53.9 413.8,18.6 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,132.6 124.4,127.3 160.5,127.5 196.7,106.6 232.9,99.2 269.1,86.3 305.3,73.9 341.5,61.3 377.6,53.9 413.8,17.8 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.44 ns | 1.83 ns | 2.46 ns | 2.18 ns |
| D38 | 2.24 ns | 3.54 ns | 4.15 ns | 1.84 ns | 3.54 ns |
| D57 | 7.16 ns | 8.08 ns | 8.09 ns | 8.08 ns | 8.09 ns |
| D76 | 9.83 ns | 4.52 ns | 8.6 ns | 9.71 ns | 4.77 ns |
| D115 | 11.2 ns | 12.7 ns | 9.73 ns | 12.7 ns | 14.4 ns |
| D153 | 20 ns | 17.1 ns | 17.6 ns | 17.8 ns | 20 ns |
| D230 | 36.3 ns | 36 ns | 32.1 ns | 31.8 ns | 32.4 ns |
| D307 | 27.4 ns | 37.3 ns | 43 ns | 47.8 ns | 42.5 ns |
| D462 | 75.9 ns | 73 ns | 73.5 ns | 54.1 ns | 83.1 ns |
| D616 | 84.9 ns | 89.5 ns | 97.3 ns | 102 ns | 95.9 ns |
| D924 | 112 ns | 102 ns | 109 ns | 108 ns | 89.4 ns |
| D1232 | 158 ns | 138 ns | 121 ns | 121 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,186.7 124.4,153.0 160.5,143.8 196.7,140.1 232.9,123.2 269.1,106.0 305.3,114.2 341.5,84.7 377.6,81.4 413.8,73.4 450.0,63.4 450.0,87.1 413.8,79.9 377.6,77.9 341.5,82.0 305.3,101.4 269.1,109.3 232.9,123.2 196.7,132.8 160.5,164.7 124.4,149.5 88.2,173.4 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,186.7 124.4,153.0 160.5,143.8 196.7,140.1 232.9,123.2 269.1,106.0 305.3,114.2 341.5,84.7 377.6,81.4 413.8,73.4 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.4 88.2,173.4 124.4,149.5 160.5,166.3 196.7,136.3 232.9,127.8 269.1,106.3 305.3,105.2 341.5,85.8 377.6,79.9 413.8,76.0 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.5 88.2,168.8 124.4,149.5 160.5,147.7 196.7,144.1 232.9,127.0 269.1,109.5 305.3,101.1 341.5,85.6 377.6,77.5 413.8,74.1 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,192.3 124.4,149.5 160.5,144.2 196.7,136.3 232.9,126.6 269.1,109.8 305.3,98.0 341.5,94.4 377.6,76.1 413.8,74.4 450.0,71.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,173.4 124.4,149.5 160.5,164.7 196.7,132.8 232.9,123.2 269.1,109.3 305.3,101.4 341.5,82.0 377.6,77.9 413.8,79.9 450.0,87.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 0.841 ns | 0.913 ns | 1.05 ns | 0.937 ns |
| D38 | 0.903 ns | 1.81 ns | 2.12 ns | 0.9 ns | 1.82 ns |
| D57 | 2.26 ns | 2.51 ns | 2.5 ns | 2.5 ns | 2.5 ns |
| D76 | 3.45 ns | 1.86 ns | 3.08 ns | 3.45 ns | 2.08 ns |
| D115 | 4.29 ns | 4.85 ns | 4.03 ns | 4.87 ns | 5.55 ns |
| D153 | 8.49 ns | 7.57 ns | 7.55 ns | 7.57 ns | 8.48 ns |
| D230 | 17.7 ns | 17.6 ns | 16.1 ns | 16.1 ns | 16.2 ns |
| D307 | 15.7 ns | 19.5 ns | 23.5 ns | 25.2 ns | 23.5 ns |
| D462 | 41.2 ns | 37.8 ns | 36.7 ns | 29.5 ns | 41.2 ns |
| D616 | 51.5 ns | 50.9 ns | 61.5 ns | 49.6 ns | 49.8 ns |
| D924 | 76.8 ns | 74.9 ns | 84.7 ns | 80.4 ns | 85 ns |
| D1232 | 107 ns | 106 ns | 95.4 ns | 98.6 ns | 67 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,162.2 124.4,142.3 160.5,133.1 196.7,128.4 232.9,113.5 269.1,97.7 305.3,100.2 341.5,79.3 377.6,74.4 413.8,65.7 450.0,58.6 450.0,68.7 413.8,63.5 377.6,75.1 341.5,79.3 305.3,91.4 269.1,99.5 232.9,113.6 196.7,122.8 160.5,144.0 124.4,140.1 88.2,147.0 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,162.2 124.4,142.3 160.5,133.1 196.7,128.4 232.9,113.5 269.1,97.7 305.3,100.2 341.5,79.3 377.6,74.4 413.8,65.7 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.8 88.2,147.1 124.4,140.0 160.5,146.5 196.7,125.7 232.9,116.1 269.1,97.7 305.3,95.5 341.5,81.1 377.6,74.7 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.0 88.2,143.6 124.4,140.1 160.5,135.5 196.7,129.7 232.9,116.1 269.1,99.6 305.3,91.5 341.5,81.7 377.6,70.5 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,162.3 124.4,140.1 160.5,133.1 196.7,125.6 232.9,116.0 269.1,99.6 305.3,89.9 341.5,86.5 377.6,75.2 413.8,64.7 450.0,60.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,140.1 160.5,144.0 196.7,122.8 232.9,113.6 269.1,99.5 305.3,91.4 341.5,79.3 377.6,75.1 413.8,63.5 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
