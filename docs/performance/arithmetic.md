# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.819 ns | 0.954 ns | 0.936 ns | 1.18 ns | 0.939 ns |
| D38 | 1.82 ns | 1.61 ns | 1.61 ns | 1.81 ns | 1.82 ns |
| D57 | 2.5 ns | 2.5 ns | 2.49 ns | 2.35 ns | 2.5 ns |
| D76 | 3.49 ns | 3.48 ns | 2.7 ns | 1.56 ns | 3.09 ns |
| D115 | 4.39 ns | 4.99 ns | 4.41 ns | 4.4 ns | 4.4 ns |
| D153 | 5.9 ns | 6.64 ns | 6.65 ns | 3.15 ns | 3.17 ns |
| D230 | 15.4 ns | 12.1 ns | 15.3 ns | 13.6 ns | 15.3 ns |
| D307 | 19.5 ns | 19.5 ns | 19.6 ns | 14.6 ns | 19.5 ns |
| D462 | 28.7 ns | 32.9 ns | 33.1 ns | 27.6 ns | 29.4 ns |
| D616 | 45.5 ns | 45.2 ns | 57 ns | 44.7 ns | 62.9 ns |
| D924 | 84.9 ns | 98.9 ns | 84.8 ns | 62.2 ns | 84.8 ns |
| D1232 | 97.1 ns | 95.3 ns | 92.3 ns | 95 ns | 67.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.1 88.2,126.1 124.4,116.8 160.5,107.1 196.7,100.5 232.9,91.9 269.1,64.2 305.3,57.3 341.5,46.1 377.6,32.8 413.8,14.8 450.0,10.9 450.0,21.6 413.8,14.8 377.6,23.4 341.5,45.5 305.3,57.3 269.1,64.3 232.9,109.9 196.7,100.4 160.5,110.7 124.4,116.8 88.2,126.0 52.0,145.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.1 88.2,126.1 124.4,116.8 160.5,107.1 196.7,100.5 232.9,91.9 269.1,64.2 305.3,57.3 341.5,46.1 377.6,32.8 413.8,14.8 450.0,10.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,144.7 88.2,129.5 124.4,116.8 160.5,107.3 196.7,96.8 232.9,88.5 269.1,71.1 305.3,57.3 341.5,42.1 377.6,33.0 413.8,10.3 450.0,11.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.2 88.2,129.5 124.4,116.9 160.5,114.6 196.7,100.4 232.9,88.5 269.1,64.3 305.3,57.2 341.5,42.0 377.6,26.3 413.8,14.8 450.0,12.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,138.6 88.2,126.2 124.4,118.6 160.5,130.5 196.7,100.5 232.9,110.1 269.1,67.7 305.3,65.7 341.5,47.3 377.6,33.3 413.8,23.8 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.2 88.2,126.0 124.4,116.8 160.5,110.7 196.7,100.4 232.9,109.9 269.1,64.3 305.3,57.3 341.5,45.5 377.6,23.4 413.8,14.8 450.0,21.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5 ns | 5.42 ns | 5.6 ns | 8.66 ns | 8.1 ns |
| D38 | 12.7 ns | 11.6 ns | 14.3 ns | 67.5 ns | 68.3 ns |
| D57 | 21.4 ns | 34.5 ns | 74.6 ns | 120 ns | 120 ns |
| D76 | 25.8 ns | 65.5 ns | 64.7 ns | 72.9 ns | 141 ns |
| D115 | 43.1 ns | 88.7 ns | 104 ns | 188 ns | 229 ns |
| D153 | 63 ns | 116 ns | 155 ns | 146 ns | 198 ns |
| D230 | 93.1 ns | 140 ns | 255 ns | 356 ns | 635 ns |
| D307 | 132 ns | 248 ns | 402 ns | 616 ns | 972 ns |
| D462 | 186 ns | 477 ns | 786 ns | 923 ns | 1.34 µs |
| D616 | 239 ns | 603 ns | 987 ns | 1.82 µs | 2.39 µs |
| D924 | 405 ns | 1.22 µs | 2.28 µs | 2.4 µs | 4.67 µs |
| D1232 | 515 ns | 1.76 µs | 2.87 µs | 4.15 µs | 5.03 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.0 88.2,154.7 124.4,143.4 160.5,139.5 196.7,128.3 232.9,120.0 269.1,111.6 305.3,104.0 341.5,96.5 377.6,91.0 413.8,79.6 450.0,74.4 450.0,24.9 413.8,26.5 377.6,41.1 341.5,53.6 305.3,60.6 269.1,69.9 232.9,95.1 196.7,92.0 160.5,102.6 124.4,106.0 88.2,118.3 52.0,164.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.0 88.2,154.7 124.4,143.4 160.5,139.5 196.7,128.3 232.9,120.0 269.1,111.6 305.3,104.0 341.5,96.5 377.6,91.0 413.8,79.6 450.0,74.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.3 88.2,156.7 124.4,133.1 160.5,119.2 196.7,112.6 232.9,106.7 269.1,102.7 305.3,90.3 341.5,76.1 377.6,71.0 413.8,55.7 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.6 88.2,152.3 124.4,116.4 160.5,119.5 196.7,109.1 232.9,100.4 269.1,89.6 305.3,79.8 341.5,65.2 377.6,60.3 413.8,42.1 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.1 88.2,118.5 124.4,106.0 160.5,116.9 196.7,96.3 232.9,101.7 269.1,82.4 305.3,70.5 341.5,61.8 377.6,47.1 413.8,41.0 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.6 88.2,118.3 124.4,106.0 160.5,102.6 196.7,92.0 232.9,95.1 269.1,69.9 305.3,60.6 341.5,53.6 377.6,41.1 413.8,26.5 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.816 ns | 2.9 ns | 3.13 ns | 4.37 ns | 5.1 ns |
| D38 | 3.95 ns | 13.9 ns | 26.1 ns | 40.8 ns | 37.8 ns |
| D57 | 4.22 ns | 21.9 ns | 35 ns | 71.8 ns | 77 ns |
| D76 | 5.65 ns | 39.6 ns | 35.2 ns | 48.2 ns | 102 ns |
| D115 | 13.3 ns | 51.4 ns | 86.1 ns | 197 ns | 230 ns |
| D153 | 18.4 ns | 59.4 ns | 121 ns | 135 ns | 200 ns |
| D230 | 28.5 ns | 101 ns | 368 ns | 464 ns | 1.04 µs |
| D307 | 54.5 ns | 185 ns | 514 ns | 954 ns | 1.47 µs |
| D462 | 91.4 ns | 462 ns | 1.33 µs | 1.45 µs | 2.43 µs |
| D616 | 94.8 ns | 674 ns | 1.53 µs | 2.76 µs | 4.19 µs |
| D924 | 155 ns | 1.59 µs | 3.24 µs | 4.56 µs | 8.33 µs |
| D1232 | 190 ns | 2.24 µs | 4.25 µs | 8.32 µs | 9.49 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,173.5 88.2,146.1 124.4,145.0 160.5,139.9 196.7,125.1 232.9,119.4 269.1,111.8 305.3,100.5 341.5,91.6 377.6,90.9 413.8,82.3 450.0,78.8 450.0,10.9 413.8,13.2 377.6,25.1 341.5,34.6 305.3,43.3 269.1,49.2 232.9,77.9 196.7,75.5 160.5,89.6 124.4,94.5 88.2,106.9 52.0,141.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,173.5 88.2,146.1 124.4,145.0 160.5,139.9 196.7,125.1 232.9,119.4 269.1,111.8 305.3,100.5 341.5,91.6 377.6,90.9 413.8,82.3 450.0,78.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,151.5 88.2,124.3 124.4,116.4 160.5,106.1 196.7,101.6 232.9,99.1 269.1,89.9 305.3,79.4 341.5,63.4 377.6,56.8 413.8,41.9 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.2 88.2,113.3 124.4,108.2 160.5,108.2 196.7,92.6 232.9,86.6 269.1,67.3 305.3,61.6 341.5,45.1 377.6,42.6 413.8,29.6 450.0,24.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,144.4 88.2,105.6 124.4,95.7 160.5,102.7 196.7,78.2 232.9,84.8 269.1,63.4 305.3,50.8 341.5,43.6 377.6,32.3 413.8,23.6 450.0,13.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.7 88.2,106.9 124.4,94.5 160.5,89.6 196.7,75.5 232.9,77.9 269.1,49.2 305.3,43.3 341.5,34.6 377.6,25.1 413.8,13.2 450.0,10.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.546 ns | 0.622 ns | 0.623 ns | 0.645 ns | 0.623 ns |
| D38 | 1.45 ns | 1.32 ns | 1.33 ns | 1.45 ns | 1.45 ns |
| D57 | 1.74 ns | 1.74 ns | 1.74 ns | 1.62 ns | 1.74 ns |
| D76 | 2.17 ns | 2.16 ns | 1.68 ns | 1.5 ns | 2.49 ns |
| D115 | 2.82 ns | 3.16 ns | 2.83 ns | 3.28 ns | 3.28 ns |
| D153 | 3.79 ns | 4.22 ns | 4.6 ns | 2.31 ns | 2.31 ns |
| D230 | 6.65 ns | 4.3 ns | 7.24 ns | 5.14 ns | 7.25 ns |
| D307 | 12.3 ns | 12.4 ns | 12.4 ns | 7.79 ns | 12.4 ns |
| D462 | 15 ns | 17 ns | 17.1 ns | 14 ns | 15.3 ns |
| D616 | 19 ns | 20.2 ns | 27.7 ns | 20.2 ns | 21.8 ns |
| D924 | 63.3 ns | 95.4 ns | 84.7 ns | 68 ns | 84.7 ns |
| D1232 | 47.1 ns | 62.1 ns | 53.7 ns | 62 ns | 40.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.9 88.2,132.6 124.4,127.3 160.5,120.9 196.7,113.3 232.9,104.8 269.1,88.5 305.3,70.7 341.5,64.9 377.6,58.1 413.8,23.3 450.0,31.8 450.0,36.5 413.8,14.8 377.6,54.1 341.5,64.3 305.3,70.5 269.1,86.0 232.9,119.0 196.7,108.9 160.5,116.9 124.4,127.3 88.2,132.7 52.0,157.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.9 88.2,132.6 124.4,127.3 160.5,120.9 196.7,113.3 232.9,104.8 269.1,88.5 305.3,70.7 341.5,64.9 377.6,58.1 413.8,23.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,121.0 196.7,110.0 232.9,101.6 269.1,101.1 305.3,70.5 341.5,61.3 377.6,56.3 413.8,11.3 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,135.2 124.4,127.3 160.5,128.3 196.7,113.2 232.9,99.2 269.1,86.0 305.3,70.5 341.5,61.2 377.6,47.2 413.8,14.8 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.0 88.2,132.7 124.4,129.4 160.5,131.7 196.7,109.0 232.9,119.1 269.1,95.9 305.3,83.9 341.5,67.0 377.6,56.4 413.8,21.2 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,132.7 124.4,127.3 160.5,116.9 196.7,108.9 232.9,119.0 269.1,86.0 305.3,70.5 341.5,64.3 377.6,54.1 413.8,14.8 450.0,36.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 ns | 1.88 ns | 1.87 ns | 3.36 ns | 2.18 ns |
| D38 | 3.54 ns | 3.25 ns | 3.25 ns | 3.56 ns | 3.54 ns |
| D57 | 8.09 ns | 8.08 ns | 8.09 ns | 7.59 ns | 8.09 ns |
| D76 | 9.84 ns | 9.7 ns | 7.41 ns | 4.36 ns | 8.71 ns |
| D115 | 12.7 ns | 14.4 ns | 12.8 ns | 12.7 ns | 12.8 ns |
| D153 | 16 ns | 20 ns | 20 ns | 8.63 ns | 8.58 ns |
| D230 | 36.6 ns | 23.9 ns | 36.2 ns | 22.8 ns | 36.2 ns |
| D307 | 48.2 ns | 48.1 ns | 48 ns | 29.2 ns | 47.8 ns |
| D462 | 73 ns | 92.5 ns | 95.1 ns | 67.7 ns | 71.4 ns |
| D616 | 82.7 ns | 79.7 ns | 89 ns | 78.7 ns | 95.4 ns |
| D924 | 109 ns | 105 ns | 95.8 ns | 64.3 ns | 88 ns |
| D1232 | 134 ns | 128 ns | 99.5 ns | 115 ns | 64.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.8 88.2,173.4 124.4,149.5 160.5,143.8 196.7,136.3 232.9,129.6 269.1,105.8 305.3,97.8 341.5,85.8 377.6,82.2 413.8,74.0 450.0,68.2 450.0,89.2 413.8,80.4 377.6,78.0 341.5,86.4 305.3,98.0 269.1,106.1 232.9,147.8 196.7,136.3 160.5,147.3 124.4,149.5 88.2,173.4 52.0,187.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.8 88.2,173.4 124.4,149.5 160.5,143.8 196.7,136.3 232.9,129.6 269.1,105.8 305.3,97.8 341.5,85.8 377.6,82.2 413.8,74.0 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.8 88.2,175.9 124.4,149.5 160.5,144.2 196.7,132.8 232.9,123.2 269.1,118.1 305.3,97.9 341.5,78.9 377.6,83.2 413.8,75.3 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,175.9 124.4,149.5 160.5,152.0 196.7,136.3 232.9,123.2 269.1,106.1 305.3,97.9 341.5,78.1 377.6,80.0 413.8,77.9 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.9 88.2,173.3 124.4,151.3 160.5,167.4 196.7,136.3 232.9,147.6 269.1,119.4 305.3,112.3 341.5,88.0 377.6,83.6 413.8,89.4 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,173.4 124.4,149.5 160.5,147.3 196.7,136.3 232.9,147.8 269.1,106.1 305.3,98.0 341.5,86.4 377.6,78.0 413.8,80.4 450.0,89.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.818 ns | 0.943 ns | 0.936 ns | 1.18 ns | 0.934 ns |
| D38 | 1.81 ns | 1.6 ns | 1.61 ns | 1.82 ns | 1.82 ns |
| D57 | 2.52 ns | 2.51 ns | 2.5 ns | 2.31 ns | 2.5 ns |
| D76 | 3.46 ns | 3.46 ns | 2.68 ns | 1.87 ns | 3.09 ns |
| D115 | 4.88 ns | 5.54 ns | 4.88 ns | 4.89 ns | 4.89 ns |
| D153 | 7.65 ns | 8.45 ns | 8.47 ns | 4.35 ns | 4.38 ns |
| D230 | 17.6 ns | 13.7 ns | 17.6 ns | 15 ns | 17.7 ns |
| D307 | 25.1 ns | 25.2 ns | 25.1 ns | 18.1 ns | 25.2 ns |
| D462 | 36.7 ns | 41.1 ns | 43 ns | 34.4 ns | 37.6 ns |
| D616 | 45.1 ns | 45.9 ns | 61.2 ns | 45.7 ns | 62.4 ns |
| D924 | 84.8 ns | 98.8 ns | 84.7 ns | 69.2 ns | 84.8 ns |
| D1232 | 99 ns | 95.2 ns | 102 ns | 95 ns | 73.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.4 88.2,147.1 124.4,140.0 160.5,133.1 196.7,125.6 232.9,115.8 269.1,97.7 305.3,90.0 341.5,81.8 377.6,77.3 413.8,63.6 450.0,60.2 450.0,66.7 413.8,63.6 377.6,70.2 341.5,81.3 305.3,89.9 269.1,97.6 232.9,127.9 196.7,125.5 160.5,135.5 124.4,140.1 88.2,147.0 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.4 88.2,147.1 124.4,140.0 160.5,133.1 196.7,125.6 232.9,115.8 269.1,97.7 305.3,90.0 341.5,81.8 377.6,77.3 413.8,63.6 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.3 88.2,149.8 124.4,140.0 160.5,133.1 196.7,122.8 232.9,113.7 269.1,103.2 305.3,89.9 341.5,79.3 377.6,76.9 413.8,60.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,140.1 160.5,138.6 196.7,125.6 232.9,113.6 269.1,97.7 305.3,90.0 341.5,78.3 377.6,70.7 413.8,63.6 450.0,59.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.5 88.2,147.0 124.4,141.8 160.5,146.5 196.7,125.5 232.9,128.1 269.1,101.2 305.3,97.1 341.5,83.2 377.6,77.0 413.8,68.0 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,140.1 160.5,135.5 196.7,125.5 232.9,127.9 269.1,97.6 305.3,89.9 341.5,81.3 377.6,70.2 413.8,63.6 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
