# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.938 ns | 1.04 ns | 0.935 ns | 1.06 ns | 0.936 ns |
| D38 | 1.61 ns | 1.83 ns | 1.62 ns | 1.61 ns | 1.82 ns |
| D57 | 2.25 ns | 2.5 ns | 2.5 ns | 2.27 ns | 2.25 ns |
| D76 | 3.08 ns | 3.08 ns | 3.49 ns | 3.08 ns | 3.09 ns |
| D115 | 4.98 ns | 5 ns | 4.99 ns | 3.34 ns | 4.41 ns |
| D153 | 4.47 ns | 5.89 ns | 6.66 ns | 6.63 ns | 5.89 ns |
| D230 | 13.8 ns | 13.9 ns | 13.9 ns | 11.9 ns | 13.9 ns |
| D307 | 18.5 ns | 18.6 ns | 19.6 ns | 18.5 ns | 14.5 ns |
| D462 | 58.3 ns | 29.8 ns | 32.7 ns | 28.9 ns | 29.2 ns |
| D616 | 45 ns | 45.8 ns | 49.6 ns | 45.4 ns | 52.2 ns |
| D924 | 86.5 ns | 81.8 ns | 98.4 ns | 85.1 ns | 85.7 ns |
| D1232 | 95 ns | 107 ns | 98.5 ns | 95.4 ns | 95.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.6 124.4,142.4 160.5,135.6 196.7,125.1 232.9,127.5 269.1,103.0 305.3,96.6 341.5,71.7 377.6,77.3 413.8,63.2 450.0,61.1 450.0,60.9 413.8,63.3 377.6,74.1 341.5,86.8 305.3,101.9 269.1,102.9 232.9,121.5 196.7,127.8 160.5,135.5 124.4,142.4 88.2,146.9 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.4 160.5,135.6 196.7,125.1 232.9,127.5 269.1,103.0 305.3,96.6 341.5,71.7 377.6,77.3 413.8,63.2 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,159.2 88.2,146.9 124.4,140.1 160.5,135.6 196.7,125.1 232.9,121.5 269.1,102.8 305.3,96.6 341.5,86.3 377.6,77.0 413.8,64.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.5 124.4,140.1 160.5,132.9 196.7,125.1 232.9,118.8 269.1,102.9 305.3,95.4 341.5,84.2 377.6,75.2 413.8,60.4 450.0,60.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,149.7 124.4,142.2 160.5,135.6 196.7,133.8 232.9,118.9 269.1,106.3 305.3,96.6 341.5,86.9 377.6,77.2 413.8,63.5 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,146.9 124.4,142.4 160.5,135.5 196.7,127.8 232.9,121.5 269.1,102.9 305.3,101.9 341.5,86.8 377.6,74.1 413.8,63.3 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.95 ns | 6.07 ns | 5.6 ns | 9.9 ns | 8.1 ns |
| D38 | 9.56 ns | 12.4 ns | 15.2 ns | 726 ns | 1.07 µs |
| D57 | 38.1 ns | 52.2 ns | 75.6 ns | 101 ns | 102 ns |
| D76 | 40.7 ns | 60.4 ns | 85.8 ns | 104 ns | 126 ns |
| D115 | 56.2 ns | 84.5 ns | 112 ns | 164 ns | 210 ns |
| D153 | 58.3 ns | 107 ns | 153 ns | 239 ns | 294 ns |
| D230 | 95.2 ns | 153 ns | 227 ns | 325 ns | 539 ns |
| D307 | 130 ns | 221 ns | 392 ns | 577 ns | 849 ns |
| D462 | 252 ns | 399 ns | 694 ns | 997 ns | 1.31 µs |
| D616 | 262 ns | 601 ns | 988 ns | 1.74 µs | 2.37 µs |
| D924 | 385 ns | 1.1 µs | 2.25 µs | 2.75 µs | 4.59 µs |
| D1232 | 501 ns | 1.89 µs | 3.47 µs | 4.13 µs | 7.09 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.3 88.2,161.0 124.4,131.0 160.5,129.5 196.7,122.5 232.9,121.7 269.1,111.1 305.3,104.2 341.5,89.9 377.6,89.1 413.8,80.7 450.0,75.0 450.0,17.5 413.8,26.9 377.6,41.3 341.5,54.1 305.3,63.6 269.1,73.4 232.9,86.6 196.7,93.9 160.5,105.0 124.4,109.6 88.2,58.6 52.0,164.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.3 88.2,161.0 124.4,131.0 160.5,129.5 196.7,122.5 232.9,121.7 269.1,111.1 305.3,104.2 341.5,89.9 377.6,89.1 413.8,80.7 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.9 88.2,155.4 124.4,124.1 160.5,121.0 196.7,113.6 232.9,108.5 269.1,100.7 305.3,92.8 341.5,79.9 377.6,71.1 413.8,58.0 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.6 88.2,150.9 124.4,116.1 160.5,113.3 196.7,107.5 232.9,100.8 269.1,92.2 305.3,80.3 341.5,67.9 377.6,60.3 413.8,42.4 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,67.0 124.4,109.7 160.5,109.2 196.7,99.2 232.9,91.1 269.1,84.4 305.3,71.9 341.5,60.1 377.6,47.9 413.8,38.0 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.6 88.2,58.6 124.4,109.6 160.5,105.0 196.7,93.9 232.9,86.6 269.1,73.4 305.3,63.6 341.5,54.1 377.6,41.3 413.8,26.9 450.0,17.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.82 ns | 3.1 ns | 5.02 ns | 5.05 ns |
| D38 | 3.49 ns | 13.7 ns | 26.3 ns | 26 ns | 29.5 ns |
| D57 | 6.24 ns | 24.3 ns | 36 ns | 71.5 ns | 72.6 ns |
| D76 | 8.1 ns | 34.1 ns | 43.6 ns | 79.8 ns | 101 ns |
| D115 | 13.5 ns | 55.9 ns | 89.7 ns | 180 ns | 227 ns |
| D153 | 14.8 ns | 54.6 ns | 121 ns | 259 ns | 354 ns |
| D230 | 27.2 ns | 122 ns | 337 ns | 446 ns | 972 ns |
| D307 | 44.3 ns | 166 ns | 515 ns | 1.02 µs | 1.33 µs |
| D462 | 86.8 ns | 405 ns | 1.32 µs | 1.72 µs | 2.39 µs |
| D616 | 88.7 ns | 672 ns | 1.71 µs | 2.69 µs | 4.18 µs |
| D924 | 157 ns | 1.47 µs | 3.19 µs | 5.41 µs | 8.27 µs |
| D1232 | 191 ns | 2.42 µs | 4.6 µs | 8.18 µs | 12.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.6 124.4,150.2 160.5,146.4 196.7,139.0 232.9,137.7 269.1,128.9 305.3,121.8 341.5,112.1 377.6,111.7 413.8,103.4 450.0,100.6 450.0,39.8 413.8,46.1 377.6,56.0 341.5,64.0 305.3,72.5 269.1,77.1 232.9,91.7 196.7,98.1 160.5,109.9 124.4,114.6 88.2,127.7 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.6 124.4,150.2 160.5,146.4 196.7,139.0 232.9,137.7 269.1,128.9 305.3,121.8 341.5,112.1 377.6,111.7 413.8,103.4 450.0,100.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.7 88.2,138.8 124.4,130.5 160.5,125.6 196.7,118.4 232.9,118.8 269.1,107.1 305.3,102.7 341.5,89.7 377.6,82.4 413.8,71.1 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,129.3 124.4,124.8 160.5,122.0 196.7,111.6 232.9,107.3 269.1,92.4 305.3,86.3 341.5,72.6 377.6,68.9 413.8,59.9 450.0,54.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,129.5 124.4,114.9 160.5,113.3 196.7,101.5 232.9,96.2 269.1,88.4 305.3,76.3 341.5,68.9 377.6,62.3 413.8,52.2 450.0,46.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,127.7 124.4,114.6 160.5,109.9 196.7,98.1 232.9,91.7 269.1,77.1 305.3,72.5 341.5,64.0 377.6,56.0 413.8,46.1 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.346 ns | 0.622 ns | 0.703 ns | 0.622 ns |
| D38 | 1.42 ns | 1.45 ns | 1.42 ns | 1.42 ns | 1.45 ns |
| D57 | 1.69 ns | 1.74 ns | 1.74 ns | 1.69 ns | 1.69 ns |
| D76 | 2.19 ns | 2.19 ns | 2.17 ns | 2.18 ns | 2.49 ns |
| D115 | 3.17 ns | 3.16 ns | 3.16 ns | 2.81 ns | 3.25 ns |
| D153 | 2.9 ns | 3.79 ns | 4.6 ns | 4.6 ns | 4.49 ns |
| D230 | 5.86 ns | 5.86 ns | 7.16 ns | 5.61 ns | 7.16 ns |
| D307 | 10.9 ns | 11.1 ns | 12.5 ns | 11.1 ns | 7.72 ns |
| D462 | 17.3 ns | 15.3 ns | 17 ns | 14.9 ns | 14.9 ns |
| D616 | 19.1 ns | 20.5 ns | 20.3 ns | 20.2 ns | 21.8 ns |
| D924 | 63.3 ns | 81.8 ns | 93.8 ns | 84.7 ns | 84.6 ns |
| D1232 | 47.2 ns | 69.8 ns | 61.9 ns | 61.7 ns | 61.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,133.3 124.4,128.2 160.5,120.7 196.7,110.0 232.9,112.5 269.1,92.1 305.3,74.2 341.5,60.8 377.6,58.0 413.8,23.3 450.0,31.7 450.0,24.0 413.8,14.8 377.6,54.1 341.5,65.1 305.3,84.2 269.1,86.3 232.9,99.9 196.7,109.2 160.5,117.0 124.4,128.2 88.2,132.6 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,133.3 124.4,128.2 160.5,120.7 196.7,110.0 232.9,112.5 269.1,92.1 305.3,74.2 341.5,60.8 377.6,58.0 413.8,23.3 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.1 88.2,132.6 124.4,127.3 160.5,120.7 196.7,110.0 232.9,104.8 269.1,92.1 305.3,73.7 341.5,64.3 377.6,55.9 413.8,15.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,133.2 124.4,127.3 160.5,120.9 196.7,110.0 232.9,99.2 269.1,86.3 305.3,70.2 341.5,61.3 377.6,56.2 413.8,11.8 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,133.3 124.4,128.2 160.5,120.7 196.7,113.4 232.9,99.1 269.1,93.4 305.3,73.7 341.5,65.1 377.6,56.3 413.8,14.8 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,128.2 160.5,117.0 196.7,109.2 232.9,99.9 269.1,86.3 305.3,84.2 341.5,65.1 377.6,54.1 413.8,14.8 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.73 ns | 1.87 ns | 2.46 ns | 2.18 ns |
| D38 | 7.05 ns | 7.87 ns | 12.4 ns | 13 ns | 16.5 ns |
| D57 | 7.17 ns | 8.1 ns | 8.09 ns | 7.17 ns | 7.17 ns |
| D76 | 8.42 ns | 8.73 ns | 9.52 ns | 8.43 ns | 8.43 ns |
| D115 | 14.4 ns | 14.1 ns | 14.1 ns | 9.18 ns | 12.5 ns |
| D153 | 12.8 ns | 15.8 ns | 20 ns | 20.1 ns | 16.5 ns |
| D230 | 31.9 ns | 32.2 ns | 32.2 ns | 27.9 ns | 32.1 ns |
| D307 | 41.3 ns | 40.2 ns | 48.3 ns | 43.2 ns | 29.5 ns |
| D462 | 101 ns | 76.9 ns | 101 ns | 72.4 ns | 77.5 ns |
| D616 | 83.2 ns | 83.4 ns | 81.4 ns | 77.8 ns | 94.2 ns |
| D924 | 113 ns | 116 ns | 113 ns | 102 ns | 119 ns |
| D1232 | 133 ns | 137 ns | 138 ns | 115 ns | 111 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,153.5 124.4,152.9 160.5,148.3 196.7,132.8 232.9,136.1 269.1,109.8 305.3,102.3 341.5,76.4 377.6,82.0 413.8,73.0 450.0,68.3 450.0,73.5 413.8,71.7 377.6,78.4 341.5,84.1 305.3,112.0 269.1,109.5 232.9,128.8 196.7,137.0 160.5,148.3 124.4,153.0 88.2,128.8 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,153.5 124.4,152.9 160.5,148.3 196.7,132.8 232.9,136.1 269.1,109.8 305.3,102.3 341.5,76.4 377.6,82.0 413.8,73.0 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,194.1 88.2,150.3 124.4,149.5 160.5,147.3 196.7,133.5 232.9,130.1 269.1,109.5 305.3,103.0 341.5,84.3 377.6,81.9 413.8,72.4 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,137.1 124.4,149.5 160.5,144.8 196.7,133.5 232.9,123.2 269.1,109.5 305.3,97.7 341.5,76.3 377.6,82.6 413.8,73.3 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,135.6 124.4,153.0 160.5,148.3 196.7,145.8 232.9,123.2 269.1,113.6 305.3,101.0 341.5,86.0 377.6,83.9 413.8,76.1 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,128.8 124.4,153.0 160.5,148.3 196.7,137.0 232.9,128.8 269.1,109.5 305.3,112.0 341.5,84.1 377.6,78.4 413.8,71.7 450.0,73.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 1.15 ns | 0.934 ns | 1.06 ns | 0.937 ns |
| D38 | 1.6 ns | 1.83 ns | 1.62 ns | 1.61 ns | 1.82 ns |
| D57 | 2.25 ns | 2.49 ns | 2.5 ns | 2.26 ns | 2.25 ns |
| D76 | 3.08 ns | 3.1 ns | 3.46 ns | 3.09 ns | 3.09 ns |
| D115 | 5.55 ns | 5.54 ns | 5.53 ns | 4.03 ns | 4.84 ns |
| D153 | 6.36 ns | 7.64 ns | 8.44 ns | 8.47 ns | 7.65 ns |
| D230 | 16.2 ns | 16.1 ns | 16.1 ns | 13.6 ns | 16.2 ns |
| D307 | 23.2 ns | 23.1 ns | 24.9 ns | 23.1 ns | 18 ns |
| D462 | 62.5 ns | 37.2 ns | 43.2 ns | 37.1 ns | 37.2 ns |
| D616 | 44.4 ns | 45.3 ns | 50.6 ns | 45.3 ns | 49.3 ns |
| D924 | 84.9 ns | 82.1 ns | 98.7 ns | 84.8 ns | 85.7 ns |
| D1232 | 95.1 ns | 107 ns | 98.3 ns | 95.1 ns | 97.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.8 124.4,142.4 160.5,135.5 196.7,122.8 232.9,119.8 269.1,99.6 305.3,91.7 341.5,70.2 377.6,77.6 413.8,63.6 450.0,61.1 450.0,60.6 413.8,63.3 377.6,75.4 341.5,81.5 305.3,97.3 269.1,99.5 232.9,115.8 196.7,125.7 160.5,135.5 124.4,142.4 88.2,147.0 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.8 124.4,142.4 160.5,135.5 196.7,122.8 232.9,119.8 269.1,99.6 305.3,91.7 341.5,70.2 377.6,77.6 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,156.9 88.2,146.9 124.4,140.2 160.5,135.4 196.7,122.8 232.9,115.8 269.1,99.6 305.3,91.8 341.5,81.4 377.6,77.2 413.8,64.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.5 124.4,140.1 160.5,133.1 196.7,122.9 232.9,113.7 269.1,99.6 305.3,90.2 341.5,78.2 377.6,74.8 413.8,60.3 450.0,60.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,142.3 160.5,135.5 196.7,129.8 232.9,113.6 269.1,103.3 305.3,91.8 341.5,81.5 377.6,77.2 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,142.4 160.5,135.5 196.7,125.7 232.9,115.8 269.1,99.5 305.3,97.3 341.5,81.5 377.6,75.4 413.8,63.3 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
