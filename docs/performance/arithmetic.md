# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.937 ns | 1.05 ns | 1.18 ns | 0.935 ns | 1.05 ns |
| D38 | 0.984 ns | 1.62 ns | 1.62 ns | 1.64 ns | 1.11 ns |
| D57 | 1.94 ns | 1.94 ns | 1.21 ns | 2.25 ns | 1.94 ns |
| D76 | 3.48 ns | 2.7 ns | 1.8 ns | 3.1 ns | 2.7 ns |
| D115 | 5.01 ns | 4.99 ns | 2.54 ns | 4.4 ns | 5 ns |
| D153 | 3.62 ns | 6.65 ns | 3.85 ns | 6.64 ns | 5.15 ns |
| D230 | 13.9 ns | 14 ns | 14 ns | 13.9 ns | 13.9 ns |
| D307 | 18.5 ns | 13.5 ns | 15.2 ns | 18.6 ns | 18.7 ns |
| D462 | 25.8 ns | 29 ns | 32 ns | 29.6 ns | 29.7 ns |
| D616 | 41.7 ns | 60.2 ns | 51.5 ns | 44.8 ns | 58.3 ns |
| D924 | 84.8 ns | 65.9 ns | 84.7 ns | 74.8 ns | 74.8 ns |
| D1232 | 79.9 ns | 95.3 ns | 103 ns | 95 ns | 67.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,160.4 124.4,145.6 160.5,132.9 196.7,125.0 232.9,132.1 269.1,102.8 305.3,96.6 341.5,89.4 377.6,79.0 413.8,63.6 450.0,64.9 450.0,68.7 413.8,66.3 377.6,71.7 341.5,86.4 305.3,96.4 269.1,102.8 232.9,124.4 196.7,125.1 160.5,138.4 124.4,145.6 88.2,157.7 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,160.4 124.4,145.6 160.5,132.9 196.7,125.0 232.9,132.1 269.1,102.8 305.3,96.6 341.5,89.4 377.6,79.0 413.8,63.6 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.5 124.4,145.7 160.5,138.4 196.7,125.1 232.9,118.9 269.1,102.7 305.3,103.5 341.5,86.9 377.6,71.0 413.8,69.0 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.5 88.2,149.6 124.4,155.8 160.5,147.2 196.7,139.7 232.9,130.7 269.1,102.7 305.3,100.9 341.5,84.7 377.6,74.4 413.8,63.6 450.0,59.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.3 124.4,142.4 160.5,135.4 196.7,127.8 232.9,118.9 269.1,102.8 305.3,96.6 341.5,86.4 377.6,77.4 413.8,66.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,157.7 124.4,145.6 160.5,138.4 196.7,125.1 232.9,124.4 269.1,102.8 305.3,96.4 341.5,86.4 377.6,71.7 413.8,66.3 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.77 ns | 6.21 ns | 5.55 ns | 8.08 ns | 8.91 ns |
| D38 | 7.42 ns | 11.4 ns | 15.1 ns | 58.3 ns | 59.5 ns |
| D57 | 19.7 ns | 26.8 ns | 38 ns | 110 ns | 101 ns |
| D76 | 25.3 ns | 51.7 ns | 51 ns | 114 ns | 119 ns |
| D115 | 46.8 ns | 88.6 ns | 62.5 ns | 188 ns | 253 ns |
| D153 | 34 ns | 115 ns | 106 ns | 264 ns | 274 ns |
| D230 | 82.5 ns | 154 ns | 231 ns | 391 ns | 570 ns |
| D307 | 117 ns | 148 ns | 311 ns | 592 ns | 889 ns |
| D462 | 149 ns | 413 ns | 550 ns | 1.07 µs | 1.46 µs |
| D616 | 202 ns | 753 ns | 1.12 µs | 1.85 µs | 1.88 µs |
| D924 | 383 ns | 791 ns | 2.28 µs | 2.55 µs | 4.25 µs |
| D1232 | 390 ns | 1.8 µs | 3.56 µs | 4.18 µs | 5.39 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.9 88.2,166.5 124.4,145.3 160.5,139.9 196.7,126.5 232.9,133.4 269.1,114.2 305.3,106.5 341.5,101.3 377.6,94.7 413.8,80.8 450.0,80.4 450.0,23.4 413.8,28.6 377.6,46.3 341.5,51.8 305.3,62.6 269.1,72.2 232.9,88.1 196.7,89.8 160.5,106.3 124.4,109.9 88.2,121.3 52.0,162.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.9 88.2,166.5 124.4,145.3 160.5,139.9 196.7,126.5 232.9,133.4 269.1,114.2 305.3,106.5 341.5,101.3 377.6,94.7 413.8,80.8 450.0,80.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.3 88.2,157.1 124.4,138.6 160.5,124.3 196.7,112.6 232.9,106.9 269.1,100.6 305.3,101.4 341.5,79.2 377.6,66.2 413.8,65.1 450.0,47.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.8 88.2,151.0 124.4,131.0 160.5,124.6 196.7,120.2 232.9,108.8 269.1,91.9 305.3,85.4 341.5,73.0 377.6,57.6 413.8,42.1 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.6 88.2,121.7 124.4,107.9 160.5,107.2 196.7,96.3 232.9,88.9 269.1,80.4 305.3,71.4 341.5,58.5 377.6,46.6 413.8,39.6 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.5 88.2,121.3 124.4,109.9 160.5,106.3 196.7,89.8 232.9,88.1 269.1,72.2 305.3,62.6 341.5,51.8 377.6,46.3 413.8,28.6 450.0,23.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.887 ns | 3.07 ns | 2.63 ns | 4.93 ns | 4.94 ns |
| D38 | 2.55 ns | 14.4 ns | 26.3 ns | 25.1 ns | 19.4 ns |
| D57 | 3.27 ns | 16.8 ns | 20.1 ns | 72.8 ns | 60.2 ns |
| D76 | 5.65 ns | 29.6 ns | 31.1 ns | 78.6 ns | 83.9 ns |
| D115 | 13.5 ns | 54.6 ns | 49.8 ns | 197 ns | 254 ns |
| D153 | 10.8 ns | 55.1 ns | 79.3 ns | 259 ns | 307 ns |
| D230 | 27.9 ns | 116 ns | 341 ns | 527 ns | 995 ns |
| D307 | 44.4 ns | 110 ns | 393 ns | 1.03 µs | 1.38 µs |
| D462 | 49.3 ns | 406 ns | 882 ns | 1.77 µs | 2.38 µs |
| D616 | 69.3 ns | 844 ns | 1.86 µs | 2.74 µs | 3.29 µs |
| D924 | 156 ns | 1.04 µs | 3.21 µs | 4.99 µs | 7.51 µs |
| D1232 | 141 ns | 2.19 µs | 4.66 µs | 8.16 µs | 9.51 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,172.1 88.2,153.7 124.4,149.4 160.5,139.9 196.7,124.7 232.9,128.7 269.1,112.2 305.3,104.1 341.5,102.3 377.6,96.4 413.8,82.3 450.0,84.1 450.0,10.9 413.8,15.0 377.6,29.3 341.5,35.0 305.3,44.4 269.1,50.1 232.9,70.5 196.7,73.8 160.5,93.0 124.4,98.8 88.2,118.5 52.0,142.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,172.1 88.2,153.7 124.4,149.4 160.5,139.9 196.7,124.7 232.9,128.7 269.1,112.2 305.3,104.1 341.5,102.3 377.6,96.4 413.8,82.3 450.0,84.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,150.5 88.2,123.7 124.4,121.0 160.5,111.2 196.7,100.5 232.9,100.4 269.1,87.4 305.3,88.3 341.5,65.7 377.6,52.9 413.8,49.3 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,113.2 124.4,117.9 160.5,110.3 196.7,102.1 232.9,94.0 269.1,68.7 305.3,66.2 341.5,52.2 377.6,39.2 413.8,29.7 450.0,23.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,142.3 88.2,114.0 124.4,95.5 160.5,94.2 196.7,78.2 232.9,73.5 269.1,61.1 305.3,49.5 341.5,40.1 377.6,32.5 413.8,22.1 450.0,13.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,142.3 88.2,118.5 124.4,98.8 160.5,93.0 196.7,73.8 232.9,70.5 269.1,50.1 305.3,44.4 341.5,35.0 377.6,29.3 413.8,15.0 450.0,10.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.703 ns | 0.395 ns | 0.622 ns | 0.703 ns |
| D38 | 0.942 ns | 1.33 ns | 1.32 ns | 1.33 ns | 0.984 ns |
| D57 | 1.35 ns | 1.35 ns | 1.05 ns | 1.87 ns | 1.35 ns |
| D76 | 2.16 ns | 1.68 ns | 1.45 ns | 2.18 ns | 2.04 ns |
| D115 | 3.17 ns | 3.17 ns | 1.9 ns | 3.28 ns | 3.55 ns |
| D153 | 2.5 ns | 4.22 ns | 2.88 ns | 4.6 ns | 3.57 ns |
| D230 | 6 ns | 6 ns | 7.18 ns | 7.18 ns | 7.18 ns |
| D307 | 10.9 ns | 6.41 ns | 9.59 ns | 11 ns | 11 ns |
| D462 | 11.8 ns | 15 ns | 14.4 ns | 15.1 ns | 10.9 ns |
| D616 | 15.9 ns | 25.4 ns | 21.8 ns | 20.2 ns | 17.9 ns |
| D924 | 63.3 ns | 63.4 ns | 84.7 ns | 75.8 ns | 75.7 ns |
| D1232 | 37.7 ns | 61.4 ns | 68.5 ns | 61.4 ns | 44.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,145.1 124.4,134.7 160.5,121.0 196.7,110.0 232.9,116.8 269.1,91.5 305.3,74.2 341.5,71.9 377.6,63.3 413.8,23.2 450.0,38.2 450.0,33.3 413.8,18.0 377.6,59.8 341.5,74.2 305.3,73.9 269.1,86.3 232.9,106.5 196.7,106.6 160.5,122.7 124.4,134.7 88.2,143.8 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,145.1 124.4,134.7 160.5,121.0 196.7,110.0 232.9,116.8 269.1,91.5 305.3,74.2 341.5,71.9 377.6,63.3 413.8,23.2 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.2 124.4,134.7 160.5,128.3 196.7,110.0 232.9,101.7 269.1,91.5 305.3,89.5 341.5,64.8 377.6,49.7 413.8,23.2 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.2 88.2,135.2 124.4,141.9 160.5,132.5 196.7,124.8 232.9,112.7 269.1,86.3 305.3,77.9 341.5,66.2 377.6,54.1 413.8,14.8 450.0,20.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,120.7 196.7,108.9 232.9,99.2 269.1,86.3 305.3,73.9 341.5,64.8 377.6,56.3 413.8,18.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,143.8 124.4,134.7 160.5,122.7 196.7,106.6 232.9,106.5 269.1,86.3 305.3,73.9 341.5,74.2 377.6,59.8 413.8,18.0 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 2.02 ns | 2.18 ns | 2.46 ns |
| D38 | 2.24 ns | 3.25 ns | 3.24 ns | 3.17 ns | 2.34 ns |
| D57 | 6.27 ns | 6.27 ns | 3.46 ns | 7.16 ns | 6.27 ns |
| D76 | 9.51 ns | 7.63 ns | 4.81 ns | 8.71 ns | 7.62 ns |
| D115 | 14.4 ns | 14.4 ns | 7.68 ns | 12.8 ns | 14.4 ns |
| D153 | 9.59 ns | 20.1 ns | 10.9 ns | 20 ns | 15.6 ns |
| D230 | 32.2 ns | 32.3 ns | 31.8 ns | 32.1 ns | 32.1 ns |
| D307 | 41.9 ns | 27 ns | 37.2 ns | 42.7 ns | 42.5 ns |
| D462 | 60.6 ns | 79.5 ns | 64.5 ns | 72 ns | 60.7 ns |
| D616 | 75.2 ns | 116 ns | 97.8 ns | 78.1 ns | 74 ns |
| D924 | 112 ns | 72.3 ns | 104 ns | 83.9 ns | 78.6 ns |
| D1232 | 99.6 ns | 141 ns | 130 ns | 115 ns | 66.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,186.7 124.4,156.8 160.5,144.8 196.7,132.8 232.9,144.5 269.1,109.5 305.3,101.9 341.5,91.2 377.6,84.9 413.8,73.5 450.0,76.8 450.0,88.4 413.8,83.6 377.6,85.4 341.5,91.1 305.3,101.4 269.1,109.6 232.9,130.5 196.7,132.8 160.5,151.2 124.4,156.8 88.2,185.4 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,186.7 124.4,156.8 160.5,144.8 196.7,132.8 232.9,144.5 269.1,109.5 305.3,101.9 341.5,91.2 377.6,84.9 413.8,73.5 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,175.9 124.4,156.8 160.5,151.2 196.7,132.8 232.9,123.2 269.1,109.4 305.3,114.6 341.5,83.3 377.6,72.3 413.8,86.1 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.7 88.2,175.9 124.4,174.1 160.5,164.5 196.7,151.0 232.9,140.8 269.1,109.8 305.3,105.3 341.5,89.4 377.6,77.3 413.8,75.5 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.6 124.4,153.0 160.5,147.3 196.7,136.3 232.9,123.2 269.1,109.5 305.3,101.3 341.5,86.2 377.6,83.8 413.8,81.8 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,185.4 124.4,156.8 160.5,151.2 196.7,132.8 232.9,130.5 269.1,109.6 305.3,101.4 341.5,91.1 377.6,85.4 413.8,83.6 450.0,88.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 1.06 ns | 1.18 ns | 0.937 ns | 1.05 ns |
| D38 | 0.917 ns | 1.61 ns | 1.61 ns | 1.61 ns | 0.988 ns |
| D57 | 1.95 ns | 1.95 ns | 1.3 ns | 2.27 ns | 1.94 ns |
| D76 | 3.45 ns | 2.68 ns | 2.09 ns | 3.09 ns | 2.68 ns |
| D115 | 5.54 ns | 5.55 ns | 3.09 ns | 4.89 ns | 5.55 ns |
| D153 | 4.77 ns | 8.43 ns | 5.57 ns | 8.43 ns | 6.55 ns |
| D230 | 16.1 ns | 16.1 ns | 16.1 ns | 16.2 ns | 16.1 ns |
| D307 | 23.1 ns | 15.7 ns | 19.5 ns | 23 ns | 23 ns |
| D462 | 32.1 ns | 36.9 ns | 36.9 ns | 36.7 ns | 35.7 ns |
| D616 | 45.5 ns | 57.9 ns | 49.3 ns | 45.9 ns | 59.2 ns |
| D924 | 84.9 ns | 71.9 ns | 84.7 ns | 75 ns | 74.7 ns |
| D1232 | 89.2 ns | 95.4 ns | 103 ns | 95.6 ns | 73.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,161.9 124.4,145.5 160.5,133.1 196.7,122.8 232.9,126.1 269.1,99.6 305.3,91.9 341.5,84.7 377.6,77.1 413.8,63.6 450.0,62.5 450.0,66.6 413.8,66.3 377.6,71.4 341.5,82.4 305.3,91.9 269.1,99.6 232.9,119.2 196.7,122.8 160.5,138.6 124.4,145.6 88.2,160.3 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,161.9 124.4,145.5 160.5,133.1 196.7,122.8 232.9,126.1 269.1,99.6 305.3,91.9 341.5,84.7 377.6,77.1 413.8,63.6 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.7 124.4,145.5 160.5,138.6 196.7,122.8 232.9,113.7 269.1,99.7 305.3,100.2 341.5,81.7 377.6,71.9 413.8,67.2 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.5 88.2,149.7 124.4,154.3 160.5,144.0 196.7,135.5 232.9,122.7 269.1,99.6 305.3,95.5 341.5,81.6 377.6,75.3 413.8,63.6 450.0,59.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.2 160.5,135.5 196.7,125.6 232.9,113.7 269.1,99.5 305.3,91.9 341.5,81.7 377.6,76.9 413.8,66.2 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,160.3 124.4,145.6 160.5,138.6 196.7,122.8 232.9,119.2 269.1,99.6 305.3,91.9 341.5,82.4 377.6,71.4 413.8,66.3 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
