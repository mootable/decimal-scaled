# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 0.936 ns | 0.934 ns | 1.05 ns | 1.05 ns |
| D38 | 1.83 ns | 1.44 ns | 1.6 ns | 1.62 ns | 1.42 ns |
| D57 | 2.25 ns | 2.25 ns | 2.49 ns | 2.5 ns | 2.25 ns |
| D76 | 3.1 ns | 3.49 ns | 3.08 ns | 3.08 ns | 3.09 ns |
| D115 | 4.99 ns | 4.99 ns | 4.99 ns | 4.42 ns | 4.99 ns |
| D153 | 5.95 ns | 6.64 ns | 6.64 ns | 5.95 ns | 6.63 ns |
| D230 | 13.8 ns | 15.3 ns | 15.4 ns | 12.4 ns | 13.9 ns |
| D307 | 18.5 ns | 18.5 ns | 16.7 ns | 18.5 ns | 19.6 ns |
| D462 | 58.5 ns | 63.1 ns | 29.2 ns | 29.7 ns | 29.6 ns |
| D616 | 45.3 ns | 33.3 ns | 51.1 ns | 62.7 ns | 67.1 ns |
| D924 | 76 ns | 75.4 ns | 84.7 ns | 56.2 ns | 84.9 ns |
| D1232 | 105 ns | 116 ns | 107 ns | 95.1 ns | 108 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,135.4 196.7,125.1 232.9,121.3 269.1,102.9 305.3,96.6 341.5,71.6 377.6,77.2 413.8,66.0 450.0,58.9 450.0,58.4 413.8,63.6 377.6,68.7 341.5,86.4 305.3,95.4 269.1,102.9 232.9,118.9 196.7,125.1 160.5,135.5 124.4,142.4 88.2,152.4 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,135.4 196.7,125.1 232.9,121.3 269.1,102.9 305.3,96.6 341.5,71.6 377.6,77.2 413.8,66.0 450.0,58.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,152.1 124.4,142.4 160.5,132.9 196.7,125.1 232.9,118.9 269.1,100.7 305.3,96.6 341.5,70.0 377.6,83.9 413.8,66.1 450.0,56.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.8 124.4,140.2 160.5,135.6 196.7,125.1 232.9,118.9 269.1,100.7 305.3,98.9 341.5,86.7 377.6,74.6 413.8,63.6 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,140.1 160.5,135.6 196.7,127.8 232.9,121.3 269.1,105.4 305.3,96.6 341.5,86.4 377.6,70.2 413.8,72.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.4 124.4,142.4 160.5,135.5 196.7,125.1 232.9,118.9 269.1,102.9 305.3,95.4 341.5,86.4 377.6,68.7 413.8,63.6 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.98 ns | 5.92 ns | 5.7 ns | 9.22 ns | 9.33 ns |
| D38 | 11.2 ns | 12.4 ns | 15.2 ns | 726 ns | 827 ns |
| D57 | 38.1 ns | 49.2 ns | 74.8 ns | 107 ns | 102 ns |
| D76 | 40.3 ns | 65.9 ns | 76 ns | 104 ns | 125 ns |
| D115 | 59.6 ns | 84.6 ns | 110 ns | 170 ns | 232 ns |
| D153 | 66.3 ns | 113 ns | 153 ns | 222 ns | 320 ns |
| D230 | 96.5 ns | 177 ns | 250 ns | 374 ns | 535 ns |
| D307 | 131 ns | 220 ns | 330 ns | 562 ns | 922 ns |
| D462 | 251 ns | 480 ns | 655 ns | 1.03 µs | 1.34 µs |
| D616 | 267 ns | 539 ns | 1.05 µs | 1.78 µs | 2.36 µs |
| D924 | 356 ns | 1.05 µs | 2.24 µs | 2.66 µs | 4.64 µs |
| D1232 | 489 ns | 1.9 µs | 3.75 µs | 4.17 µs | 7.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.2 88.2,157.6 124.4,131.0 160.5,129.7 196.7,121.2 232.9,118.9 269.1,110.8 305.3,104.2 341.5,90.0 377.6,88.7 413.8,82.5 450.0,75.5 450.0,15.4 413.8,26.7 377.6,41.4 341.5,53.7 305.3,61.8 269.1,73.6 232.9,84.8 196.7,91.7 160.5,105.1 124.4,109.5 88.2,64.1 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,157.6 124.4,131.0 160.5,129.7 196.7,121.2 232.9,118.9 269.1,110.8 305.3,104.2 341.5,90.0 377.6,88.7 413.8,82.5 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,155.4 124.4,125.4 160.5,119.1 196.7,113.6 232.9,107.3 269.1,97.6 305.3,92.9 341.5,75.9 377.6,73.4 413.8,59.0 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,151.0 124.4,116.3 160.5,116.0 196.7,108.0 232.9,100.8 269.1,90.1 305.3,84.1 341.5,69.2 377.6,58.9 413.8,42.5 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,67.0 124.4,108.5 160.5,109.2 196.7,98.5 232.9,92.7 269.1,81.4 305.3,72.5 341.5,59.5 377.6,47.5 413.8,38.7 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,64.1 124.4,109.5 160.5,105.1 196.7,91.7 232.9,84.8 269.1,73.6 305.3,61.8 341.5,53.7 377.6,41.4 413.8,26.7 450.0,15.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 3 ns | 3.1 ns | 5.01 ns | 4.96 ns |
| D38 | 3.94 ns | 12.2 ns | 26.4 ns | 25.1 ns | 23 ns |
| D57 | 6.55 ns | 20.8 ns | 35.9 ns | 78.2 ns | 71.8 ns |
| D76 | 8.1 ns | 39.9 ns | 42.2 ns | 79.8 ns | 103 ns |
| D115 | 13.7 ns | 57.3 ns | 89.7 ns | 194 ns | 252 ns |
| D153 | 18.7 ns | 57.2 ns | 121 ns | 234 ns | 396 ns |
| D230 | 27.2 ns | 126 ns | 371 ns | 488 ns | 979 ns |
| D307 | 44.4 ns | 169 ns | 435 ns | 1.05 µs | 1.45 µs |
| D462 | 87.2 ns | 449 ns | 1.26 µs | 1.76 µs | 2.45 µs |
| D616 | 92.9 ns | 588 ns | 1.85 µs | 2.7 µs | 4.18 µs |
| D924 | 149 ns | 1.46 µs | 3.21 µs | 4.94 µs | 8.29 µs |
| D1232 | 182 ns | 2.38 µs | 5.06 µs | 8.05 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,156.8 124.4,149.5 160.5,146.4 196.7,138.8 232.9,134.3 269.1,128.9 305.3,121.8 341.5,112.0 377.6,111.1 413.8,104.2 450.0,101.3 450.0,38.3 413.8,46.0 377.6,56.0 341.5,63.7 305.3,71.3 269.1,77.0 232.9,90.1 196.7,96.6 160.5,109.6 124.4,114.8 88.2,131.3 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,156.8 124.4,149.5 160.5,146.4 196.7,138.8 232.9,134.3 269.1,128.9 305.3,121.8 341.5,112.0 377.6,111.1 413.8,104.2 450.0,101.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.8 88.2,140.4 124.4,132.7 160.5,123.3 196.7,118.1 232.9,118.1 269.1,106.7 305.3,102.4 341.5,88.3 377.6,84.4 413.8,71.1 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,129.3 124.4,124.8 160.5,122.5 196.7,111.6 232.9,107.3 269.1,91.0 305.3,88.7 341.5,73.4 377.6,67.8 413.8,59.8 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,130.0 124.4,113.6 160.5,113.3 196.7,100.4 232.9,97.7 269.1,87.1 305.3,76.0 341.5,68.5 377.6,62.3 413.8,53.6 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,131.3 124.4,114.8 160.5,109.6 196.7,96.6 232.9,90.1 269.1,77.0 305.3,71.3 341.5,63.7 377.6,56.0 413.8,46.0 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.622 ns | 0.622 ns | 0.703 ns | 0.703 ns |
| D38 | 1.45 ns | 1.15 ns | 1.32 ns | 1.33 ns | 1.12 ns |
| D57 | 1.87 ns | 1.87 ns | 1.74 ns | 1.74 ns | 1.87 ns |
| D76 | 2.09 ns | 2.17 ns | 2.1 ns | 2.09 ns | 2.49 ns |
| D115 | 3.17 ns | 3.17 ns | 3.17 ns | 3.25 ns | 3.55 ns |
| D153 | 3.82 ns | 4.22 ns | 4.6 ns | 4.29 ns | 4.6 ns |
| D230 | 5.86 ns | 6.65 ns | 7.24 ns | 5.22 ns | 7.16 ns |
| D307 | 10.9 ns | 11.1 ns | 10.6 ns | 11.1 ns | 12.5 ns |
| D462 | 17.7 ns | 41.8 ns | 14.9 ns | 15.3 ns | 15.3 ns |
| D616 | 18.9 ns | 15 ns | 21.9 ns | 19.9 ns | 22.5 ns |
| D924 | 55.5 ns | 75.5 ns | 84.7 ns | 60.7 ns | 84.8 ns |
| D1232 | 47.5 ns | 70 ns | 70 ns | 61.4 ns | 70.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,121.9 196.7,110.0 232.9,104.5 269.1,92.1 305.3,74.2 341.5,60.1 377.6,58.2 413.8,27.0 450.0,31.6 450.0,20.2 413.8,14.8 377.6,53.2 341.5,64.4 305.3,70.2 269.1,86.3 232.9,99.2 196.7,106.6 160.5,116.9 124.4,125.2 88.2,139.9 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,121.9 196.7,110.0 232.9,104.5 269.1,92.1 305.3,74.2 341.5,60.1 377.6,58.2 413.8,27.0 450.0,31.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,139.2 124.4,125.2 160.5,120.9 196.7,110.0 232.9,101.7 269.1,88.5 305.3,73.7 341.5,35.2 377.6,64.8 413.8,18.1 450.0,20.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,121.8 196.7,110.0 232.9,99.1 269.1,86.0 305.3,75.0 341.5,65.0 377.6,53.9 413.8,14.8 450.0,20.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.1 124.4,127.3 160.5,122.0 196.7,109.2 232.9,101.2 269.1,95.5 305.3,73.7 341.5,64.3 377.6,56.8 413.8,24.5 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,139.9 124.4,125.2 160.5,116.9 196.7,106.6 232.9,99.2 269.1,86.3 305.3,70.2 341.5,64.4 377.6,53.2 413.8,14.8 450.0,20.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 1.87 ns | 2.46 ns | 2.46 ns |
| D38 | 8.46 ns | 6.61 ns | 12.4 ns | 12.6 ns | 12.8 ns |
| D57 | 7.2 ns | 7.16 ns | 8.09 ns | 8.08 ns | 7.17 ns |
| D76 | 8.72 ns | 9.74 ns | 8.72 ns | 8.71 ns | 8.72 ns |
| D115 | 14.4 ns | 14.1 ns | 14.1 ns | 12.4 ns | 14.1 ns |
| D153 | 17.2 ns | 20 ns | 20 ns | 16.2 ns | 20.1 ns |
| D230 | 32.3 ns | 41.4 ns | 35.9 ns | 23 ns | 32.3 ns |
| D307 | 41.2 ns | 40.1 ns | 43 ns | 43.1 ns | 47.9 ns |
| D462 | 100 ns | 120 ns | 75.1 ns | 73.4 ns | 78 ns |
| D616 | 84.6 ns | 65.4 ns | 96.3 ns | 83 ns | 97.1 ns |
| D924 | 116 ns | 104 ns | 111 ns | 63.5 ns | 97.6 ns |
| D1232 | 141 ns | 150 ns | 130 ns | 115 ns | 128 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,148.2 124.4,152.9 160.5,147.3 196.7,132.8 232.9,127.7 269.1,109.4 305.3,102.3 341.5,76.7 377.6,81.5 413.8,72.3 450.0,66.6 450.0,69.6 413.8,77.4 377.6,77.5 341.5,83.9 305.3,98.0 269.1,109.4 232.9,123.2 196.7,133.5 160.5,147.3 124.4,153.0 88.2,136.1 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,148.2 124.4,152.9 160.5,147.3 196.7,132.8 232.9,127.7 269.1,109.4 305.3,102.3 341.5,76.7 377.6,81.5 413.8,72.3 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,155.3 124.4,153.0 160.5,144.1 196.7,133.5 232.9,123.2 269.1,102.2 305.3,103.1 341.5,71.3 377.6,89.0 413.8,75.6 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,137.1 124.4,149.5 160.5,147.3 196.7,133.5 232.9,123.2 269.1,106.3 305.3,101.1 341.5,85.0 377.6,77.8 413.8,73.5 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,136.6 124.4,149.5 160.5,147.3 196.7,137.0 232.9,129.4 269.1,119.2 305.3,101.0 341.5,85.6 377.6,82.1 413.8,89.8 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,136.1 124.4,153.0 160.5,147.3 196.7,133.5 232.9,123.2 269.1,109.4 305.3,98.0 341.5,83.9 377.6,77.5 413.8,77.4 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 0.937 ns | 0.935 ns | 1.05 ns | 1.05 ns |
| D38 | 1.83 ns | 1.44 ns | 1.6 ns | 1.62 ns | 1.41 ns |
| D57 | 2.25 ns | 2.25 ns | 2.5 ns | 2.5 ns | 2.25 ns |
| D76 | 3.08 ns | 3.46 ns | 3.08 ns | 3.09 ns | 3.08 ns |
| D115 | 5.54 ns | 5.55 ns | 5.55 ns | 4.84 ns | 5.57 ns |
| D153 | 7.64 ns | 8.45 ns | 8.44 ns | 7.63 ns | 8.47 ns |
| D230 | 16.1 ns | 17.6 ns | 17.6 ns | 13.9 ns | 16.1 ns |
| D307 | 23.5 ns | 23.5 ns | 21.7 ns | 23.5 ns | 25.2 ns |
| D462 | 62.2 ns | 67.2 ns | 37 ns | 37.3 ns | 37.4 ns |
| D616 | 45.1 ns | 35.5 ns | 48.9 ns | 63.2 ns | 67 ns |
| D924 | 76.1 ns | 76 ns | 84.8 ns | 60.9 ns | 84.9 ns |
| D1232 | 106 ns | 115 ns | 106 ns | 95.5 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,135.5 196.7,122.8 232.9,115.9 269.1,99.6 305.3,91.5 341.5,70.3 377.6,77.3 413.8,65.9 450.0,58.8 450.0,58.6 413.8,63.6 377.6,68.7 341.5,81.4 305.3,90.0 269.1,99.6 232.9,113.6 196.7,122.7 160.5,135.5 124.4,142.4 88.2,152.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,135.5 196.7,122.8 232.9,115.9 269.1,99.6 305.3,91.5 341.5,70.3 377.6,77.3 413.8,65.9 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,152.0 124.4,142.4 160.5,133.0 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.5 341.5,68.6 377.6,82.5 413.8,66.0 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.8 124.4,140.1 160.5,135.6 196.7,122.8 232.9,113.7 269.1,97.7 305.3,93.2 341.5,81.6 377.6,75.5 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.9 88.2,149.5 124.4,140.1 160.5,135.5 196.7,125.8 232.9,115.9 269.1,102.8 305.3,91.4 341.5,81.4 377.6,70.0 413.8,70.8 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,142.4 160.5,135.5 196.7,122.7 232.9,113.6 269.1,99.6 305.3,90.0 341.5,81.4 377.6,68.7 413.8,63.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
