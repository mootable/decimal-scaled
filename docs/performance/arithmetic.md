# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.823 ns | 1.06 ns | 1.06 ns | 1.05 ns | 0.935 ns |
| D38 | 1.62 ns | 1.63 ns | 1.61 ns | 1.82 ns | 1.82 ns |
| D57 | 2.25 ns | 2.25 ns | 3.15 ns | 2.25 ns | 2.5 ns |
| D76 | 3.49 ns | 2.31 ns | 3.09 ns | 3.48 ns | 3.08 ns |
| D115 | 5 ns | 5 ns | 4.98 ns | 4.4 ns | 2.34 ns |
| D153 | 6.62 ns | 5.16 ns | 6.63 ns | 5.94 ns | 6.64 ns |
| D230 | 15.4 ns | 13.8 ns | 15.4 ns | 13.8 ns | 13.9 ns |
| D307 | 18.5 ns | 18.6 ns | 18.5 ns | 19.6 ns | 19.6 ns |
| D462 | 29.7 ns | 29.6 ns | 32.6 ns | 33 ns | 29 ns |
| D616 | 45.4 ns | 44.9 ns | 60.6 ns | 54.6 ns | 53.2 ns |
| D924 | 56 ns | 74.9 ns | 74.9 ns | 85.9 ns | 97.9 ns |
| D1232 | 107 ns | 95.5 ns | 107 ns | 107 ns | 110 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.2 88.2,149.5 124.4,142.4 160.5,132.8 196.7,125.1 232.9,119.0 269.1,100.7 305.3,96.6 341.5,86.4 377.6,77.2 413.8,72.6 450.0,58.6 450.0,57.8 413.8,60.5 377.6,73.7 341.5,86.9 305.3,95.4 269.1,102.8 232.9,118.9 196.7,141.5 160.5,135.5 124.4,140.1 88.2,147.0 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.2 88.2,149.5 124.4,142.4 160.5,132.8 196.7,125.1 232.9,119.0 269.1,100.7 305.3,96.6 341.5,86.4 377.6,77.2 413.8,72.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.4 124.4,142.4 160.5,141.8 196.7,125.1 232.9,124.4 269.1,102.9 305.3,96.6 341.5,86.4 377.6,77.4 413.8,66.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,135.1 160.5,135.5 196.7,125.1 232.9,118.9 269.1,100.7 305.3,96.6 341.5,84.3 377.6,70.9 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.4 160.5,132.9 196.7,127.8 232.9,121.3 269.1,103.0 305.3,95.4 341.5,84.0 377.6,73.1 413.8,63.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,140.1 160.5,135.5 196.7,141.5 232.9,118.9 269.1,102.8 305.3,95.4 341.5,86.9 377.6,73.7 413.8,60.5 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.04 ns | 6.31 ns | 6.34 ns | 9.24 ns | 8.64 ns |
| D38 | 9.53 ns | 11 ns | 15.2 ns | 809 ns | 1.07 µs |
| D57 | 38 ns | 49.2 ns | 65.1 ns | 100 ns | 106 ns |
| D76 | 40.9 ns | 57 ns | 76.2 ns | 105 ns | 125 ns |
| D115 | 60.4 ns | 85.7 ns | 109 ns | 168 ns | 141 ns |
| D153 | 68 ns | 89.4 ns | 152 ns | 221 ns | 319 ns |
| D230 | 111 ns | 153 ns | 249 ns | 367 ns | 535 ns |
| D307 | 131 ns | 220 ns | 356 ns | 603 ns | 937 ns |
| D462 | 208 ns | 407 ns | 699 ns | 1.03 µs | 1.34 µs |
| D616 | 261 ns | 600 ns | 1.05 µs | 1.79 µs | 2.36 µs |
| D924 | 280 ns | 1.05 µs | 2.03 µs | 2.79 µs | 4.66 µs |
| D1232 | 570 ns | 1.71 µs | 3.74 µs | 4.61 µs | 7.82 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.9 88.2,161.0 124.4,131.0 160.5,129.4 196.7,120.9 232.9,118.4 269.1,107.7 305.3,104.2 341.5,94.1 377.6,89.2 413.8,87.7 450.0,72.2 450.0,15.3 413.8,26.6 377.6,41.4 341.5,53.7 305.3,61.4 269.1,73.6 232.9,84.8 196.7,102.5 160.5,105.1 124.4,108.6 88.2,58.6 52.0,163.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.9 88.2,161.0 124.4,131.0 160.5,129.4 196.7,120.9 232.9,118.4 269.1,107.7 305.3,104.2 341.5,94.1 377.6,89.2 413.8,87.7 450.0,72.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,158.0 124.4,125.4 160.5,122.2 196.7,113.3 232.9,112.4 269.1,100.8 305.3,92.9 341.5,79.5 377.6,71.1 413.8,58.8 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.9 88.2,150.9 124.4,119.3 160.5,115.9 196.7,108.1 232.9,100.9 269.1,90.2 305.3,82.4 341.5,67.8 377.6,58.9 413.8,44.6 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.7 88.2,64.6 124.4,109.9 160.5,108.9 196.7,98.7 232.9,92.8 269.1,81.8 305.3,71.0 341.5,59.5 377.6,47.4 413.8,37.7 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.2 88.2,58.6 124.4,108.6 160.5,105.1 196.7,102.5 232.9,84.8 269.1,73.6 305.3,61.4 341.5,53.7 377.6,41.4 413.8,26.6 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.816 ns | 3.06 ns | 3.2 ns | 5.03 ns | 5.09 ns |
| D38 | 3.49 ns | 13.6 ns | 26.3 ns | 33.8 ns | 29.6 ns |
| D57 | 6.55 ns | 20.8 ns | 32.9 ns | 71.5 ns | 77.3 ns |
| D76 | 5.64 ns | 35.2 ns | 42.2 ns | 85.4 ns | 103 ns |
| D115 | 13.6 ns | 57.5 ns | 89.8 ns | 194 ns | 133 ns |
| D153 | 16.9 ns | 44.2 ns | 121 ns | 234 ns | 397 ns |
| D230 | 27.6 ns | 122 ns | 371 ns | 517 ns | 959 ns |
| D307 | 44.4 ns | 169 ns | 459 ns | 1.07 µs | 1.47 µs |
| D462 | 85.4 ns | 415 ns | 1.31 µs | 1.77 µs | 2.44 µs |
| D616 | 93.4 ns | 642 ns | 1.85 µs | 2.72 µs | 4.23 µs |
| D924 | 113 ns | 1.47 µs | 2.95 µs | 5.42 µs | 8.32 µs |
| D1232 | 202 ns | 2.2 µs | 5.03 µs | 8.93 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,158.6 124.4,149.4 160.5,151.6 196.7,138.9 232.9,135.8 269.1,128.6 305.3,121.8 341.5,112.3 377.6,111.0 413.8,108.2 450.0,99.8 450.0,38.4 413.8,46.0 377.6,55.8 341.5,63.8 305.3,71.1 269.1,77.3 232.9,90.0 196.7,105.9 160.5,109.6 124.4,113.7 88.2,127.6 52.0,153.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,158.6 124.4,149.4 160.5,151.6 196.7,138.9 232.9,135.8 269.1,128.6 305.3,121.8 341.5,112.3 377.6,111.0 413.8,108.2 450.0,99.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.5 88.2,138.8 124.4,132.7 160.5,125.1 196.7,118.0 232.9,121.8 269.1,107.2 305.3,102.4 341.5,89.4 377.6,83.1 413.8,71.1 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,129.3 124.4,126.1 160.5,122.5 196.7,111.6 232.9,107.3 269.1,91.0 305.3,88.0 341.5,72.8 377.6,67.8 413.8,61.0 450.0,53.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,125.7 124.4,114.9 160.5,112.3 196.7,100.4 232.9,97.7 269.1,86.2 305.3,75.6 341.5,68.4 377.6,62.2 413.8,52.2 450.0,45.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.1 88.2,127.6 124.4,113.7 160.5,109.6 196.7,105.9 232.9,90.0 269.1,77.3 305.3,71.1 341.5,63.8 377.6,55.8 413.8,46.0 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.545 ns | 0.703 ns | 0.703 ns | 0.703 ns | 0.622 ns |
| D38 | 1.33 ns | 1.33 ns | 1.33 ns | 1.45 ns | 1.45 ns |
| D57 | 1.87 ns | 1.87 ns | 1.87 ns | 1.87 ns | 1.74 ns |
| D76 | 2.17 ns | 1.79 ns | 2.1 ns | 2.17 ns | 2.49 ns |
| D115 | 3.17 ns | 3.17 ns | 3.17 ns | 3.25 ns | 1.8 ns |
| D153 | 4.22 ns | 3.27 ns | 4.6 ns | 4.29 ns | 4.6 ns |
| D230 | 6.65 ns | 5.86 ns | 7.24 ns | 7.17 ns | 7.16 ns |
| D307 | 10.9 ns | 11.1 ns | 11.1 ns | 12.5 ns | 12.5 ns |
| D462 | 15.1 ns | 15.3 ns | 16.7 ns | 14.9 ns | 14.9 ns |
| D616 | 19 ns | 20.2 ns | 21.8 ns | 27.7 ns | 22 ns |
| D924 | 38.2 ns | 75.8 ns | 75.6 ns | 84.8 ns | 96 ns |
| D1232 | 54.4 ns | 61.4 ns | 69.8 ns | 69.7 ns | 69.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.9 88.2,135.1 124.4,125.2 160.5,121.0 196.7,110.0 232.9,101.6 269.1,88.5 305.3,74.2 341.5,64.7 377.6,58.0 413.8,37.8 450.0,27.6 450.0,20.4 413.8,11.2 377.6,53.9 341.5,65.1 305.3,70.2 269.1,86.3 232.9,99.1 196.7,126.3 160.5,116.9 124.4,127.3 88.2,132.6 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.9 88.2,135.1 124.4,125.2 160.5,121.0 196.7,110.0 232.9,101.6 269.1,88.5 305.3,74.2 341.5,64.7 377.6,58.0 413.8,37.8 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,126.4 196.7,110.0 232.9,109.0 269.1,92.2 305.3,73.7 341.5,64.4 377.6,56.4 413.8,18.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,121.9 196.7,110.0 232.9,99.1 269.1,86.0 305.3,73.7 341.5,61.8 377.6,54.1 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,125.2 160.5,120.9 196.7,109.2 232.9,101.1 269.1,86.3 305.3,70.2 341.5,65.1 377.6,47.2 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,127.3 160.5,116.9 196.7,126.3 232.9,99.1 269.1,86.3 305.3,70.2 341.5,65.1 377.6,53.9 413.8,11.2 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 ns | 2.11 ns | 2.11 ns | 2.46 ns | 2.18 ns |
| D38 | 7.18 ns | 7.18 ns | 12.4 ns | 19.2 ns | 16.5 ns |
| D57 | 7.16 ns | 7.17 ns | 7.18 ns | 7.17 ns | 8.09 ns |
| D76 | 9.52 ns | 6.7 ns | 8.72 ns | 9.68 ns | 8.44 ns |
| D115 | 14.4 ns | 14.1 ns | 14.1 ns | 12.4 ns | 6.65 ns |
| D153 | 20.7 ns | 15.5 ns | 20.1 ns | 16.2 ns | 20.1 ns |
| D230 | 36.6 ns | 32.2 ns | 36.3 ns | 31.9 ns | 31.8 ns |
| D307 | 41.2 ns | 40.1 ns | 44.1 ns | 47.9 ns | 48.2 ns |
| D462 | 74.5 ns | 72.9 ns | 94.1 ns | 74.8 ns | 77.5 ns |
| D616 | 83.2 ns | 81.6 ns | 95.7 ns | 89.7 ns | 95.1 ns |
| D924 | 74.3 ns | 104 ns | 102 ns | 103 ns | 112 ns |
| D1232 | 144 ns | 126 ns | 137 ns | 127 ns | 134 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,152.9 124.4,153.0 160.5,144.8 196.7,132.8 232.9,122.2 269.1,105.8 305.3,102.4 341.5,85.2 377.6,82.0 413.8,85.2 450.0,66.2 450.0,68.3 413.8,73.4 377.6,78.1 341.5,84.0 305.3,97.8 269.1,109.8 232.9,123.2 196.7,155.1 160.5,148.2 124.4,149.5 88.2,128.8 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,152.9 124.4,153.0 160.5,144.8 196.7,132.8 232.9,122.2 269.1,105.8 305.3,102.4 341.5,85.2 377.6,82.0 413.8,85.2 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,152.9 124.4,153.0 160.5,154.9 196.7,133.5 232.9,130.6 269.1,109.5 305.3,103.1 341.5,85.8 377.6,82.6 413.8,75.6 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,137.1 124.4,152.9 160.5,147.3 196.7,133.5 232.9,123.2 269.1,106.0 305.3,100.4 341.5,78.4 377.6,77.9 413.8,76.2 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,124.4 124.4,153.0 160.5,144.3 196.7,137.0 232.9,129.3 269.1,109.7 305.3,98.0 341.5,85.1 377.6,79.8 413.8,75.9 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,128.8 124.4,149.5 160.5,148.2 196.7,155.1 232.9,123.2 269.1,109.8 305.3,97.8 341.5,84.0 377.6,78.1 413.8,73.4 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.819 ns | 1.06 ns | 1.06 ns | 1.06 ns | 0.935 ns |
| D38 | 1.62 ns | 1.61 ns | 1.61 ns | 1.82 ns | 1.82 ns |
| D57 | 2.25 ns | 2.25 ns | 3.2 ns | 2.25 ns | 2.5 ns |
| D76 | 3.45 ns | 2.45 ns | 3.09 ns | 3.46 ns | 3.09 ns |
| D115 | 5.55 ns | 5.55 ns | 5.54 ns | 4.84 ns | 2.92 ns |
| D153 | 8.47 ns | 6.56 ns | 8.48 ns | 7.64 ns | 8.45 ns |
| D230 | 17.6 ns | 16.1 ns | 17.7 ns | 16.1 ns | 16.1 ns |
| D307 | 23.5 ns | 23.5 ns | 23.5 ns | 25.2 ns | 25.1 ns |
| D462 | 37.4 ns | 37.4 ns | 42.4 ns | 41.4 ns | 37.2 ns |
| D616 | 45.9 ns | 46.1 ns | 62.6 ns | 55 ns | 49.5 ns |
| D924 | 60.9 ns | 76 ns | 75.7 ns | 84.9 ns | 98.3 ns |
| D1232 | 106 ns | 95.4 ns | 106 ns | 106 ns | 108 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.3 88.2,149.6 124.4,142.4 160.5,133.1 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.5 341.5,81.4 377.6,76.9 413.8,70.8 450.0,58.7 450.0,58.4 413.8,60.4 377.6,75.3 341.5,81.5 305.3,90.0 269.1,99.6 232.9,113.6 196.7,136.7 160.5,135.5 124.4,140.1 88.2,147.0 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.3 88.2,149.6 124.4,142.4 160.5,133.1 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.5 341.5,81.4 377.6,76.9 413.8,70.8 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.4 160.5,140.6 196.7,122.8 232.9,119.2 269.1,99.6 305.3,91.5 341.5,81.4 377.6,76.8 413.8,66.0 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,134.8 160.5,135.5 196.7,122.8 232.9,113.6 269.1,97.6 305.3,91.4 341.5,78.6 377.6,70.2 413.8,66.0 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.0 124.4,142.4 160.5,133.1 196.7,125.8 232.9,115.8 269.1,99.6 305.3,89.9 341.5,79.1 377.6,73.0 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,140.1 160.5,135.5 196.7,136.7 232.9,113.6 269.1,99.6 305.3,90.0 341.5,81.5 377.6,75.3 413.8,60.4 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
