# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 0.938 ns | 0.935 ns | 0.935 ns | 0.937 ns |
| D38 | 1.81 ns | 1.45 ns | 1.23 ns | 1.61 ns | 1.61 ns |
| D57 | 1.39 ns | 3.23 ns | 1.74 ns | 2.5 ns | 2.26 ns |
| D76 | 3.08 ns | 3.09 ns | 3.48 ns | 2.71 ns | 1.81 ns |
| D115 | 5 ns | 4.98 ns | 2.42 ns | 3.88 ns | 4.4 ns |
| D153 | 5.9 ns | 5.91 ns | 3.48 ns | 5.9 ns | 6.64 ns |
| D230 | 13.9 ns | 13.9 ns | 7.97 ns | 13.9 ns | 13.9 ns |
| D307 | 19.6 ns | 18.6 ns | 19.6 ns | 15.2 ns | 18.5 ns |
| D462 | 29 ns | 33.1 ns | 28.9 ns | 32.9 ns | 28.8 ns |
| D616 | 45.4 ns | 50.2 ns | 50 ns | 47.3 ns | 55 ns |
| D924 | 89.5 ns | 76.4 ns | 84.8 ns | 70.9 ns | 75.2 ns |
| D1232 | 95 ns | 106 ns | 95.9 ns | 70.7 ns | 85.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,147.1 124.4,152.9 160.5,135.6 196.7,125.0 232.9,121.5 269.1,102.9 305.3,95.4 341.5,86.9 377.6,77.2 413.8,62.4 450.0,61.1 450.0,63.5 413.8,66.2 377.6,73.0 341.5,87.0 305.3,96.6 269.1,102.9 232.9,118.9 196.7,127.8 160.5,147.2 124.4,142.3 88.2,149.6 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,147.1 124.4,152.9 160.5,135.6 196.7,125.0 232.9,121.5 269.1,102.9 305.3,95.4 341.5,86.9 377.6,77.2 413.8,62.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,152.0 124.4,134.5 160.5,135.5 196.7,125.1 232.9,121.4 269.1,102.9 305.3,96.6 341.5,84.0 377.6,75.0 413.8,65.9 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,155.6 124.4,148.0 160.5,132.9 196.7,140.8 232.9,132.9 269.1,114.9 305.3,95.4 341.5,87.0 377.6,75.0 413.8,63.6 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,138.4 196.7,130.6 232.9,121.5 269.1,102.9 305.3,100.9 341.5,84.1 377.6,76.3 413.8,67.5 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.3 160.5,147.2 196.7,127.8 232.9,118.9 269.1,102.9 305.3,96.6 341.5,87.0 377.6,73.0 413.8,66.2 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.08 ns | 5.93 ns | 5.98 ns | 7.69 ns | 7.84 ns |
| D38 | 12.7 ns | 12.7 ns | 12.2 ns | 58.7 ns | 59.9 ns |
| D57 | 18.9 ns | 33.2 ns | 52 ns | 120 ns | 112 ns |
| D76 | 26.2 ns | 58.6 ns | 83.9 ns | 102 ns | 108 ns |
| D115 | 46.6 ns | 88.5 ns | 62.2 ns | 156 ns | 232 ns |
| D153 | 55 ns | 110 ns | 98.4 ns | 246 ns | 353 ns |
| D230 | 83.3 ns | 155 ns | 143 ns | 413 ns | 574 ns |
| D307 | 132 ns | 226 ns | 399 ns | 503 ns | 880 ns |
| D462 | 216 ns | 457 ns | 706 ns | 1.06 µs | 1.33 µs |
| D616 | 197 ns | 536 ns | 1.03 µs | 1.63 µs | 2.16 µs |
| D924 | 366 ns | 924 ns | 2.28 µs | 1.86 µs | 4.15 µs |
| D1232 | 538 ns | 1.95 µs | 3.54 µs | 4.48 µs | 6.87 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.8 88.2,154.8 124.4,146.2 160.5,139.1 196.7,126.6 232.9,123.0 269.1,114.0 305.3,104.0 341.5,93.3 377.6,95.3 413.8,81.8 450.0,73.5 450.0,18.2 413.8,29.1 377.6,43.3 341.5,53.8 305.3,62.8 269.1,72.1 232.9,82.6 196.7,91.8 160.5,108.4 124.4,107.5 88.2,121.1 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.8 88.2,154.8 124.4,146.2 160.5,139.1 196.7,126.6 232.9,123.0 269.1,114.0 305.3,104.0 341.5,93.3 377.6,95.3 413.8,81.8 450.0,73.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,154.8 124.4,133.9 160.5,121.6 196.7,112.7 232.9,108.0 269.1,100.5 305.3,92.3 341.5,77.0 377.6,73.5 413.8,61.7 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.2 88.2,155.6 124.4,124.2 160.5,113.8 196.7,120.3 232.9,110.3 269.1,102.3 305.3,80.0 341.5,67.6 377.6,59.3 413.8,42.1 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.7 88.2,121.6 124.4,106.0 160.5,109.6 196.7,100.4 232.9,90.4 269.1,79.2 305.3,74.9 341.5,58.8 377.6,49.4 413.8,46.5 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,121.1 124.4,107.5 160.5,108.4 196.7,91.8 232.9,82.6 269.1,72.1 305.3,62.8 341.5,53.8 377.6,43.3 413.8,29.1 450.0,18.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.883 ns | 2.87 ns | 3.11 ns | 4.92 ns | 5.06 ns |
| D38 | 3.96 ns | 12.3 ns | 21.1 ns | 26.1 ns | 26.4 ns |
| D57 | 3.36 ns | 21.5 ns | 28.5 ns | 76.8 ns | 72.5 ns |
| D76 | 7.8 ns | 34.8 ns | 45.7 ns | 64.9 ns | 73.2 ns |
| D115 | 13.7 ns | 51.3 ns | 50.7 ns | 165 ns | 231 ns |
| D153 | 18.1 ns | 49.3 ns | 67.5 ns | 235 ns | 396 ns |
| D230 | 28.1 ns | 117 ns | 189 ns | 519 ns | 982 ns |
| D307 | 53.9 ns | 167 ns | 509 ns | 875 ns | 1.4 µs |
| D462 | 99 ns | 448 ns | 1.28 µs | 1.79 µs | 2.51 µs |
| D616 | 71.6 ns | 577 ns | 1.78 µs | 2.38 µs | 3.9 µs |
| D924 | 144 ns | 1.24 µs | 3.23 µs | 3.54 µs | 7.69 µs |
| D1232 | 182 ns | 2.37 µs | 4.7 µs | 8.13 µs | 12.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.5 88.2,156.7 124.4,159.1 160.5,146.9 196.7,138.8 232.9,134.8 269.1,128.4 305.3,119.0 341.5,110.1 377.6,114.8 413.8,104.7 450.0,101.4 450.0,40.4 413.8,47.1 377.6,57.0 341.5,63.4 305.3,71.8 269.1,76.9 232.9,90.1 196.7,97.9 160.5,114.5 124.4,114.7 88.2,129.3 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.5 88.2,156.7 124.4,159.1 160.5,146.9 196.7,138.8 232.9,134.8 269.1,128.4 305.3,119.0 341.5,110.1 377.6,114.8 413.8,104.7 450.0,101.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,140.3 124.4,132.3 160.5,125.3 196.7,119.7 232.9,120.2 269.1,107.8 305.3,102.5 341.5,88.3 377.6,84.6 413.8,73.6 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,132.5 124.4,128.2 160.5,121.3 196.7,119.8 232.9,115.7 269.1,100.8 305.3,86.4 341.5,73.1 377.6,68.3 413.8,59.7 450.0,54.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,129.4 124.4,113.8 160.5,116.3 196.7,102.7 232.9,97.6 269.1,86.2 305.3,78.6 341.5,68.2 377.6,64.1 413.8,58.4 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,129.3 124.4,114.7 160.5,114.5 196.7,97.9 232.9,90.1 269.1,76.9 305.3,71.8 341.5,63.4 377.6,57.0 413.8,47.1 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.622 ns | 0.623 ns | 0.623 ns | 0.622 ns |
| D38 | 1.45 ns | 1.15 ns | 1.09 ns | 1.32 ns | 1.32 ns |
| D57 | 1.26 ns | 1.87 ns | 1.55 ns | 1.74 ns | 1.87 ns |
| D76 | 2.1 ns | 2.09 ns | 2.16 ns | 1.68 ns | 1.73 ns |
| D115 | 3.16 ns | 3.17 ns | 1.93 ns | 2.75 ns | 3.28 ns |
| D153 | 3.82 ns | 3.82 ns | 2.67 ns | 4.3 ns | 4.6 ns |
| D230 | 6 ns | 6 ns | 3.62 ns | 7.17 ns | 7.18 ns |
| D307 | 12.3 ns | 11 ns | 12.4 ns | 9.59 ns | 11 ns |
| D462 | 15 ns | 17 ns | 15 ns | 15 ns | 15 ns |
| D616 | 15.3 ns | 18.1 ns | 26.2 ns | 21.6 ns | 20 ns |
| D924 | 74.2 ns | 74.5 ns | 84.7 ns | 47.5 ns | 76.3 ns |
| D1232 | 47 ns | 69.8 ns | 61.4 ns | 44.6 ns | 50.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,132.7 124.4,136.6 160.5,121.9 196.7,110.0 232.9,104.5 269.1,91.5 305.3,70.7 341.5,65.0 377.6,64.3 413.8,18.6 450.0,31.9 450.0,29.7 413.8,17.8 377.6,56.6 341.5,64.9 305.3,73.9 269.1,86.3 232.9,99.1 196.7,108.9 160.5,127.4 124.4,125.2 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,132.7 124.4,136.6 160.5,121.9 196.7,110.0 232.9,104.5 269.1,91.5 305.3,70.7 341.5,65.0 377.6,64.3 413.8,18.6 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,139.2 124.4,125.2 160.5,121.9 196.7,110.0 232.9,104.5 269.1,91.5 305.3,73.9 341.5,61.3 377.6,59.6 413.8,18.5 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,140.7 124.4,130.6 160.5,121.0 196.7,124.3 232.9,114.9 269.1,106.1 305.3,70.5 341.5,65.0 377.6,48.7 413.8,14.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,135.2 124.4,127.3 160.5,128.3 196.7,114.0 232.9,101.1 269.1,86.3 305.3,77.9 341.5,64.9 377.6,54.4 413.8,31.6 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,127.4 196.7,108.9 232.9,99.1 269.1,86.3 305.3,73.9 341.5,64.9 377.6,56.6 413.8,17.8 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 1.87 ns | 2.18 ns | 2.18 ns |
| D38 | 3.54 ns | 3.56 ns | 2.58 ns | 3.2 ns | 3.2 ns |
| D57 | 4.19 ns | 7.18 ns | 4.86 ns | 8.09 ns | 7.16 ns |
| D76 | 8.5 ns | 8.72 ns | 9.83 ns | 7.41 ns | 4.8 ns |
| D115 | 14.4 ns | 14.4 ns | 7.21 ns | 11.2 ns | 12.8 ns |
| D153 | 16.1 ns | 16 ns | 19.7 ns | 16.1 ns | 20.1 ns |
| D230 | 32.1 ns | 32.1 ns | 18 ns | 31.8 ns | 32 ns |
| D307 | 48.1 ns | 40.8 ns | 48.4 ns | 37.1 ns | 42.5 ns |
| D462 | 78.4 ns | 86.7 ns | 80.2 ns | 73 ns | 71.4 ns |
| D616 | 74.6 ns | 77.2 ns | 87.5 ns | 90.5 ns | 84.3 ns |
| D924 | 119 ns | 95 ns | 108 ns | 65 ns | 99.8 ns |
| D1232 | 141 ns | 144 ns | 125 ns | 84.8 ns | 112 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,173.4 124.4,168.5 160.5,148.0 196.7,132.8 232.9,129.6 269.1,109.6 305.3,97.9 341.5,83.7 377.6,85.2 413.8,71.7 450.0,66.8 450.0,73.4 413.8,76.7 377.6,81.6 341.5,86.4 305.3,101.4 269.1,109.6 232.9,123.2 196.7,136.2 160.5,164.6 124.4,153.0 88.2,176.4 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,173.4 124.4,168.5 160.5,148.0 196.7,132.8 232.9,129.6 269.1,109.6 305.3,97.9 341.5,83.7 377.6,85.2 413.8,71.7 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,173.3 124.4,152.9 160.5,147.3 196.7,132.8 232.9,129.8 269.1,109.6 305.3,102.6 341.5,80.8 377.6,84.1 413.8,78.1 450.0,66.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,182.6 124.4,164.2 160.5,143.8 196.7,152.8 232.9,123.7 269.1,126.3 305.3,97.7 341.5,83.1 377.6,80.5 413.8,74.3 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.4 124.4,149.5 160.5,152.0 196.7,140.1 232.9,129.5 269.1,109.8 305.3,105.4 341.5,85.8 377.6,79.6 413.8,89.1 450.0,81.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.4 124.4,153.0 160.5,164.6 196.7,136.2 232.9,123.2 269.1,109.6 305.3,101.4 341.5,86.4 377.6,81.6 413.8,76.7 450.0,73.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 ns | 1.25 ns | 1.25 ns | 1.25 ns | 1.25 ns |
| D38 | 1.81 ns | 1.45 ns | 1.11 ns | 1.61 ns | 1.61 ns |
| D57 | 1.53 ns | 3.22 ns | 1.9 ns | 2.51 ns | 2.28 ns |
| D76 | 3.09 ns | 3.09 ns | 3.44 ns | 2.68 ns | 2.09 ns |
| D115 | 5.54 ns | 5.57 ns | 3.2 ns | 4.3 ns | 4.84 ns |
| D153 | 7.58 ns | 7.57 ns | 4.72 ns | 7.55 ns | 8.47 ns |
| D230 | 16.1 ns | 16.1 ns | 9.17 ns | 16.1 ns | 16.2 ns |
| D307 | 24.9 ns | 23.1 ns | 24.9 ns | 19.3 ns | 23.1 ns |
| D462 | 36.9 ns | 43.1 ns | 37 ns | 41.4 ns | 37 ns |
| D616 | 49.7 ns | 50 ns | 54.7 ns | 51.8 ns | 57.1 ns |
| D924 | 89.8 ns | 84.3 ns | 85 ns | 67.6 ns | 74.8 ns |
| D1232 | 95.5 ns | 106 ns | 96.3 ns | 77.6 ns | 94.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.6 88.2,192.8 124.4,197.7 160.5,177.3 196.7,160.5 232.9,151.4 269.1,129.5 305.3,116.9 341.5,105.5 377.6,96.9 413.8,79.8 450.0,78.0 450.0,78.3 413.8,85.1 377.6,92.9 341.5,105.5 305.3,119.1 269.1,129.4 232.9,148.1 196.7,164.3 160.5,188.6 124.4,186.2 88.2,196.1 52.0,203.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.6 88.2,192.8 124.4,197.7 160.5,177.3 196.7,160.5 232.9,151.4 269.1,129.5 305.3,116.9 341.5,105.5 377.6,96.9 413.8,79.8 450.0,78.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,203.6 88.2,199.3 124.4,176.1 160.5,177.4 196.7,160.3 232.9,151.4 269.1,129.5 305.3,119.1 341.5,101.0 377.6,96.8 413.8,81.6 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,203.7 88.2,207.0 124.4,191.4 160.5,174.2 196.7,176.4 232.9,165.1 269.1,145.8 305.3,117.0 341.5,105.5 377.6,94.2 413.8,81.4 450.0,77.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,203.6 88.2,196.2 124.4,183.4 160.5,181.5 196.7,167.8 232.9,151.5 269.1,129.5 305.3,124.3 341.5,102.2 377.6,95.7 413.8,88.0 450.0,84.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,203.6 88.2,196.1 124.4,186.2 160.5,188.6 196.7,164.3 232.9,148.1 269.1,129.4 305.3,119.1 341.5,105.5 377.6,92.9 413.8,85.1 450.0,78.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
