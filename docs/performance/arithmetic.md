# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.938 ns | 0.938 ns | 1.05 ns | 1.06 ns | 1.05 ns |
| D38 | 1.62 ns | 0.982 ns | 1.61 ns | 1.6 ns | 1.83 ns |
| D57 | 2.09 ns | 2.17 ns | 1.54 ns | 1.93 ns | 2.25 ns |
| D76 | 3.49 ns | 1.56 ns | 2.71 ns | 3.1 ns | 2.71 ns |
| D115 | 4.41 ns | 2.43 ns | 4.45 ns | 4.99 ns | 4.99 ns |
| D153 | 5.89 ns | 6.62 ns | 3.88 ns | 5.9 ns | 5.9 ns |
| D230 | 13.8 ns | 15.3 ns | 15.3 ns | 14 ns | 15.4 ns |
| D307 | 19.6 ns | 15.2 ns | 19.6 ns | 19.6 ns | 19.6 ns |
| D462 | 28.9 ns | 32.7 ns | 33.2 ns | 29.9 ns | 32.8 ns |
| D616 | 50.4 ns | 45.5 ns | 45.2 ns | 45.3 ns | 58.8 ns |
| D924 | 88.4 ns | 55.5 ns | 76.3 ns | 76.3 ns | 67.1 ns |
| D1232 | 76.8 ns | 96.6 ns | 78.1 ns | 95.1 ns | 115 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.6 124.4,144.0 160.5,132.9 196.7,127.8 232.9,121.5 269.1,102.9 305.3,95.4 341.5,86.9 377.6,74.9 413.8,62.7 450.0,65.7 450.0,57.0 413.8,68.7 377.6,71.5 341.5,84.2 305.3,95.4 269.1,100.6 232.9,121.5 196.7,125.1 160.5,138.4 124.4,142.3 88.2,146.9 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.6 124.4,144.0 160.5,132.9 196.7,127.8 232.9,121.5 269.1,102.9 305.3,95.4 341.5,86.9 377.6,74.9 413.8,62.7 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,160.4 124.4,143.2 160.5,150.3 196.7,140.7 232.9,119.0 269.1,100.8 305.3,100.9 341.5,84.3 377.6,77.1 413.8,72.8 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,150.6 160.5,138.3 196.7,127.6 232.9,130.6 269.1,100.7 305.3,95.4 341.5,84.0 377.6,77.2 413.8,65.9 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,145.7 160.5,135.4 196.7,125.1 232.9,121.5 269.1,102.7 305.3,95.4 341.5,86.2 377.6,77.2 413.8,65.9 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.3 160.5,138.4 196.7,125.1 232.9,121.5 269.1,100.6 305.3,95.4 341.5,84.2 377.6,71.5 413.8,68.7 450.0,57.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.88 ns | 5.93 ns | 6.3 ns | 9.87 ns | 9.96 ns |
| D38 | 9.59 ns | 8.46 ns | 15.1 ns | 726 ns | 1.07 µs |
| D57 | 35.5 ns | 49.7 ns | 48.6 ns | 87.4 ns | 104 ns |
| D76 | 41.3 ns | 34.1 ns | 65.6 ns | 106 ns | 104 ns |
| D115 | 54.9 ns | 51.2 ns | 104 ns | 186 ns | 231 ns |
| D153 | 66.4 ns | 113 ns | 107 ns | 221 ns | 298 ns |
| D230 | 94.5 ns | 179 ns | 250 ns | 340 ns | 592 ns |
| D307 | 143 ns | 190 ns | 393 ns | 609 ns | 930 ns |
| D462 | 214 ns | 453 ns | 699 ns | 995 ns | 1.41 µs |
| D616 | 257 ns | 599 ns | 1.01 µs | 1.77 µs | 2.37 µs |
| D924 | 392 ns | 959 ns | 2.03 µs | 2.61 µs | 3.53 µs |
| D1232 | 410 ns | 1.7 µs | 2.51 µs | 4.27 µs | 7.88 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.5 88.2,160.9 124.4,132.5 160.5,129.2 196.7,123.0 232.9,118.9 269.1,111.2 305.3,102.3 341.5,93.4 377.6,89.5 413.8,80.3 450.0,79.3 450.0,15.2 413.8,32.6 377.6,41.3 341.5,52.5 305.3,61.6 269.1,71.4 232.9,86.3 196.7,91.9 160.5,109.1 124.4,109.2 88.2,58.6 52.0,160.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.5 88.2,160.9 124.4,132.5 160.5,129.2 196.7,123.0 232.9,118.9 269.1,111.2 305.3,102.3 341.5,93.4 377.6,89.5 413.8,80.3 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,163.6 124.4,125.2 160.5,133.4 196.7,124.5 232.9,107.4 269.1,97.3 305.3,96.1 341.5,77.2 377.6,71.1 413.8,60.9 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.0 88.2,151.0 124.4,125.7 160.5,119.1 196.7,109.2 232.9,108.5 269.1,90.1 305.3,80.3 341.5,67.8 377.6,59.8 413.8,44.7 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,67.0 124.4,112.9 160.5,108.8 196.7,96.6 232.9,92.8 269.1,83.4 305.3,70.8 341.5,60.1 377.6,47.6 413.8,39.2 450.0,28.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.1 88.2,58.6 124.4,109.2 160.5,109.1 196.7,91.9 232.9,86.3 269.1,71.4 305.3,61.6 341.5,52.5 377.6,41.3 413.8,32.6 450.0,15.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.87 ns | 3.2 ns | 4.99 ns | 5.02 ns |
| D38 | 3.5 ns | 10.9 ns | 27.4 ns | 25.1 ns | 49.7 ns |
| D57 | 5.91 ns | 19.5 ns | 25.3 ns | 60.8 ns | 71.8 ns |
| D76 | 5.63 ns | 20.7 ns | 33.8 ns | 78.8 ns | 84 ns |
| D115 | 13.4 ns | 25.6 ns | 97.3 ns | 214 ns | 253 ns |
| D153 | 18.4 ns | 57.6 ns | 80.1 ns | 235 ns | 355 ns |
| D230 | 27.7 ns | 126 ns | 371 ns | 475 ns | 1.04 µs |
| D307 | 54.4 ns | 145 ns | 514 ns | 1.09 µs | 1.48 µs |
| D462 | 75 ns | 449 ns | 1.3 µs | 1.72 µs | 2.61 µs |
| D616 | 89 ns | 656 ns | 1.74 µs | 2.71 µs | 4.18 µs |
| D924 | 158 ns | 1.31 µs | 2.96 µs | 4.95 µs | 5.9 µs |
| D1232 | 131 ns | 2.19 µs | 3.58 µs | 7.93 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,150.9 160.5,151.6 196.7,139.0 232.9,134.5 269.1,128.6 305.3,118.8 341.5,114.2 377.6,111.7 413.8,103.4 450.0,106.1 450.0,38.4 413.8,51.0 377.6,55.9 341.5,62.8 305.3,71.0 269.1,76.1 232.9,91.6 196.7,96.5 160.5,112.5 124.4,114.8 88.2,120.1 52.0,153.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,150.9 160.5,151.6 196.7,139.0 232.9,134.5 269.1,128.6 305.3,118.8 341.5,114.2 377.6,111.7 413.8,103.4 450.0,106.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,142.0 124.4,133.7 160.5,132.8 196.7,129.7 232.9,118.0 269.1,106.7 305.3,104.6 341.5,88.3 377.6,82.8 413.8,72.8 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,128.7 124.4,129.9 160.5,125.7 196.7,110.4 232.9,113.2 269.1,91.0 305.3,86.3 341.5,72.8 377.6,68.7 413.8,60.9 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,130.0 124.4,117.2 160.5,113.5 196.7,99.0 232.9,97.7 269.1,87.4 305.3,75.4 341.5,68.8 377.6,62.2 413.8,53.5 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,120.1 124.4,114.8 160.5,112.5 196.7,96.5 232.9,91.6 269.1,76.1 305.3,71.0 341.5,62.8 377.6,55.9 413.8,51.0 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.622 ns | 0.703 ns | 0.703 ns | 0.703 ns |
| D38 | 1.41 ns | 0.934 ns | 1.42 ns | 1.42 ns | 1.45 ns |
| D57 | 1.55 ns | 1.57 ns | 1.37 ns | 1.35 ns | 1.68 ns |
| D76 | 2.16 ns | 1.4 ns | 1.67 ns | 2.18 ns | 2.03 ns |
| D115 | 2.83 ns | 1.52 ns | 2.83 ns | 3.54 ns | 3.55 ns |
| D153 | 3.79 ns | 4.22 ns | 2.88 ns | 4.29 ns | 4.29 ns |
| D230 | 5.86 ns | 6.65 ns | 7.24 ns | 5.48 ns | 7.24 ns |
| D307 | 12.3 ns | 9.61 ns | 12.4 ns | 12.3 ns | 12.4 ns |
| D462 | 15 ns | 16.6 ns | 16.9 ns | 15.3 ns | 17 ns |
| D616 | 17.9 ns | 20.2 ns | 20.2 ns | 19.9 ns | 21.6 ns |
| D924 | 63.8 ns | 58.7 ns | 77 ns | 77.1 ns | 72.4 ns |
| D1232 | 39.7 ns | 61.4 ns | 55.4 ns | 61.5 ns | 73.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,133.3 124.4,130.6 160.5,121.1 196.7,113.2 232.9,104.8 269.1,92.2 305.3,70.8 341.5,64.9 377.6,59.8 413.8,23.0 450.0,36.7 450.0,19.1 413.8,19.4 377.6,54.3 341.5,61.3 305.3,70.5 269.1,86.0 232.9,101.2 196.7,106.6 160.5,122.8 124.4,128.2 88.2,132.6 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,133.3 124.4,130.6 160.5,121.1 196.7,113.2 232.9,104.8 269.1,92.2 305.3,70.8 341.5,64.9 377.6,59.8 413.8,23.0 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,145.3 124.4,130.2 160.5,133.7 196.7,131.2 232.9,101.7 269.1,88.5 305.3,77.8 341.5,61.9 377.6,56.3 413.8,25.4 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,133.3 124.4,134.2 160.5,128.4 196.7,113.2 232.9,112.7 269.1,86.0 305.3,70.5 341.5,61.4 377.6,56.3 413.8,17.6 450.0,27.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,133.3 124.4,134.7 160.5,120.7 196.7,106.7 232.9,101.2 269.1,94.1 305.3,70.6 341.5,64.4 377.6,56.8 413.8,17.5 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,128.2 160.5,122.8 196.7,106.6 232.9,101.2 269.1,86.0 305.3,70.5 341.5,61.3 377.6,54.3 413.8,19.4 450.0,19.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.86 ns | 1.87 ns | 2.11 ns | 2.46 ns | 2.46 ns |
| D38 | 7 ns | 5.06 ns | 12.5 ns | 13.2 ns | 16.4 ns |
| D57 | 6.61 ns | 6.71 ns | 4.92 ns | 6.27 ns | 7.17 ns |
| D76 | 9.84 ns | 4.62 ns | 7.62 ns | 8.43 ns | 7.38 ns |
| D115 | 12.8 ns | 6.43 ns | 12.4 ns | 14.1 ns | 14.1 ns |
| D153 | 17.1 ns | 20.1 ns | 10.9 ns | 16.3 ns | 16.6 ns |
| D230 | 32.4 ns | 43.9 ns | 36 ns | 23.2 ns | 36.4 ns |
| D307 | 48.1 ns | 37.3 ns | 48 ns | 47.9 ns | 52.2 ns |
| D462 | 77.4 ns | 87.2 ns | 89.3 ns | 72.9 ns | 82.7 ns |
| D616 | 79.8 ns | 79.4 ns | 78.1 ns | 77.8 ns | 92.2 ns |
| D924 | 123 ns | 69.5 ns | 97.9 ns | 102 ns | 74.8 ns |
| D1232 | 97.6 ns | 136 ns | 90.3 ns | 119 ns | 137 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,153.6 124.4,155.3 160.5,143.8 196.7,136.2 232.9,127.8 269.1,109.3 305.3,97.9 341.5,84.1 377.6,83.2 413.8,70.8 450.0,77.4 450.0,67.6 413.8,85.1 377.6,79.0 341.5,82.2 305.3,95.5 269.1,105.9 232.9,128.7 196.7,133.5 160.5,152.1 124.4,153.0 88.2,128.9 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,153.6 124.4,155.3 160.5,143.8 196.7,136.2 232.9,127.8 269.1,109.3 305.3,97.9 341.5,84.1 377.6,83.2 413.8,70.8 450.0,77.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,163.1 124.4,154.9 160.5,165.7 196.7,156.1 232.9,123.2 269.1,100.5 305.3,105.2 341.5,80.6 377.6,83.4 413.8,87.2 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,136.8 124.4,163.8 160.5,151.2 196.7,137.0 232.9,140.9 269.1,106.3 305.3,97.9 341.5,79.9 377.6,83.8 413.8,77.3 450.0,79.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,135.3 124.4,156.8 160.5,148.3 196.7,133.5 232.9,129.2 269.1,118.9 305.3,98.0 341.5,85.8 377.6,83.9 413.8,76.2 450.0,71.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,128.9 124.4,153.0 160.5,152.1 196.7,133.5 232.9,128.7 269.1,105.9 305.3,95.5 341.5,82.2 377.6,79.0 413.8,85.1 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 0.934 ns | 1.06 ns | 1.06 ns | 1.05 ns |
| D38 | 1.61 ns | 0.901 ns | 1.61 ns | 1.62 ns | 1.82 ns |
| D57 | 2.08 ns | 2.19 ns | 1.68 ns | 1.94 ns | 2.25 ns |
| D76 | 3.45 ns | 1.82 ns | 2.68 ns | 3.09 ns | 2.68 ns |
| D115 | 4.84 ns | 3.02 ns | 4.85 ns | 5.56 ns | 5.57 ns |
| D153 | 7.66 ns | 8.44 ns | 5.23 ns | 7.63 ns | 7.62 ns |
| D230 | 16.1 ns | 17.6 ns | 17.7 ns | 15.5 ns | 17.7 ns |
| D307 | 25.1 ns | 19.5 ns | 25.1 ns | 25.1 ns | 25.2 ns |
| D462 | 37.1 ns | 42.8 ns | 43 ns | 37.4 ns | 43.2 ns |
| D616 | 50.1 ns | 45.1 ns | 45 ns | 44.4 ns | 61.9 ns |
| D924 | 88.8 ns | 60.9 ns | 75.7 ns | 76.2 ns | 73.9 ns |
| D1232 | 85.3 ns | 96.7 ns | 88.4 ns | 95.2 ns | 112 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.6 124.4,144.1 160.5,133.1 196.7,125.7 232.9,115.8 269.1,99.6 305.3,90.0 341.5,81.5 377.6,75.0 413.8,62.6 450.0,63.5 450.0,57.5 413.8,66.6 377.6,70.4 341.5,78.2 305.3,89.9 269.1,97.6 232.9,115.9 196.7,122.7 160.5,138.6 124.4,142.4 88.2,147.0 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.6 124.4,144.1 160.5,133.1 196.7,125.7 232.9,115.8 269.1,99.6 305.3,90.0 341.5,81.5 377.6,75.0 413.8,62.6 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,162.3 124.4,142.9 160.5,147.0 196.7,136.0 232.9,113.7 269.1,97.7 305.3,95.4 341.5,78.4 377.6,77.3 413.8,70.8 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,148.8 160.5,138.6 196.7,125.7 232.9,124.1 269.1,97.7 305.3,90.0 341.5,78.3 377.6,77.3 413.8,66.1 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,145.6 160.5,135.5 196.7,122.8 232.9,115.9 269.1,100.5 305.3,90.0 341.5,81.4 377.6,77.6 413.8,65.9 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.0 124.4,142.4 160.5,138.6 196.7,122.7 232.9,115.9 269.1,97.6 305.3,89.9 341.5,78.2 377.6,70.4 413.8,66.6 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
