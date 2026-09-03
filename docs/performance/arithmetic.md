# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.937 ns | 1.06 ns | 1.14 ns | 1.05 ns | 0.934 ns |
| D38 | 1.82 ns | 1.11 ns | 1.64 ns | 1.01 ns | 1.45 ns |
| D57 | 2.5 ns | 2.25 ns | 2.25 ns | 1.22 ns | 2.25 ns |
| D76 | 3.48 ns | 2.09 ns | 3.48 ns | 3.48 ns | 3.08 ns |
| D115 | 4.41 ns | 4.4 ns | 4.4 ns | 4.41 ns | 4.4 ns |
| D153 | 5.9 ns | 4.49 ns | 6.64 ns | 5.89 ns | 5.15 ns |
| D230 | 15.3 ns | 13.9 ns | 15.4 ns | 15.3 ns | 13.9 ns |
| D307 | 12 ns | 10.1 ns | 19.6 ns | 15.1 ns | 14.6 ns |
| D462 | 29.3 ns | 32.5 ns | 28.8 ns | 29.2 ns | 32.9 ns |
| D616 | 57.5 ns | 45.3 ns | 53.9 ns | 38.5 ns | 60.1 ns |
| D924 | 55.9 ns | 84.8 ns | 74.5 ns | 74.6 ns | 61.4 ns |
| D1232 | 84.1 ns | 94.9 ns | 71.3 ns | 95.2 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,147.0 124.4,140.1 160.5,132.9 196.7,127.8 232.9,121.5 269.1,100.7 305.3,106.0 341.5,86.7 377.6,72.0 413.8,72.6 450.0,63.8 450.0,58.6 413.8,70.6 377.6,71.0 341.5,84.2 305.3,101.8 269.1,102.9 232.9,124.4 196.7,127.8 160.5,135.6 124.4,142.4 88.2,151.9 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,147.0 124.4,140.1 160.5,132.9 196.7,127.8 232.9,121.5 269.1,100.7 305.3,106.0 341.5,86.7 377.6,72.0 413.8,72.6 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.7 88.2,157.8 124.4,142.3 160.5,144.0 196.7,127.8 232.9,127.4 269.1,102.8 305.3,109.9 341.5,84.4 377.6,77.2 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,149.3 124.4,142.4 160.5,132.9 196.7,127.8 232.9,118.9 269.1,100.7 305.3,95.4 341.5,87.0 377.6,73.4 413.8,66.4 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,159.7 124.4,155.6 160.5,132.9 196.7,127.8 232.9,121.5 269.1,100.7 305.3,101.0 341.5,86.7 377.6,80.7 413.8,66.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,151.9 124.4,142.4 160.5,135.6 196.7,127.8 232.9,124.4 269.1,102.9 305.3,101.8 341.5,84.2 377.6,71.0 413.8,70.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.79 ns | 6.2 ns | 5.4 ns | 8.79 ns | 7.79 ns |
| D38 | 12.5 ns | 8.54 ns | 15.4 ns | 56.1 ns | 65.7 ns |
| D57 | 22.6 ns | 33.2 ns | 68 ns | 72.5 ns | 113 ns |
| D76 | 25.4 ns | 43.7 ns | 83.8 ns | 120 ns | 141 ns |
| D115 | 47.8 ns | 82.5 ns | 104 ns | 186 ns | 229 ns |
| D153 | 55.1 ns | 102 ns | 154 ns | 242 ns | 274 ns |
| D230 | 95 ns | 121 ns | 256 ns | 447 ns | 574 ns |
| D307 | 75.8 ns | 125 ns | 404 ns | 481 ns | 912 ns |
| D462 | 187 ns | 462 ns | 687 ns | 1.06 µs | 1.5 µs |
| D616 | 257 ns | 487 ns | 1.04 µs | 1.33 µs | 2.4 µs |
| D924 | 240 ns | 1.22 µs | 2.07 µs | 2.54 µs | 3.59 µs |
| D1232 | 416 ns | 1.76 µs | 3.15 µs | 4.15 µs | 7.78 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.9 88.2,155.1 124.4,142.3 160.5,139.7 196.7,126.0 232.9,122.9 269.1,111.1 305.3,116.0 341.5,96.4 377.6,89.5 413.8,91.0 450.0,79.0 450.0,15.4 413.8,32.3 377.6,41.0 341.5,51.2 305.3,62.0 269.1,72.1 232.9,88.1 196.7,92.0 160.5,102.5 124.4,107.3 88.2,119.1 52.0,165.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.9 88.2,155.1 124.4,142.3 160.5,139.7 196.7,126.0 232.9,122.9 269.1,111.1 305.3,116.0 341.5,96.4 377.6,89.5 413.8,91.0 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,163.4 124.4,134.0 160.5,128.0 196.7,114.2 232.9,109.6 269.1,105.9 305.3,105.1 341.5,76.8 377.6,75.6 413.8,55.7 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.4 88.2,150.7 124.4,118.4 160.5,113.8 196.7,109.2 232.9,100.6 269.1,89.6 305.3,79.7 341.5,68.2 377.6,59.2 413.8,44.2 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.8 88.2,122.5 124.4,117.0 160.5,106.0 196.7,96.5 232.9,90.8 269.1,77.5 305.3,75.9 341.5,58.7 377.6,53.9 413.8,39.8 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.4 88.2,119.1 124.4,107.3 160.5,102.5 196.7,92.0 232.9,88.1 269.1,72.1 305.3,62.0 341.5,51.2 377.6,41.0 413.8,32.3 450.0,15.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 3.07 ns | 2.54 ns | 5 ns | 5.21 ns |
| D38 | 3.94 ns | 11.2 ns | 26.5 ns | 18.6 ns | 24.9 ns |
| D57 | 4.22 ns | 21.5 ns | 33.2 ns | 46.7 ns | 72.7 ns |
| D76 | 5.64 ns | 27.7 ns | 45.6 ns | 83.3 ns | 102 ns |
| D115 | 13.2 ns | 46.1 ns | 88.7 ns | 195 ns | 231 ns |
| D153 | 18.1 ns | 46.6 ns | 120 ns | 238 ns | 307 ns |
| D230 | 28 ns | 93.3 ns | 367 ns | 570 ns | 983 ns |
| D307 | 22.9 ns | 92.1 ns | 508 ns | 754 ns | 1.32 µs |
| D462 | 93.1 ns | 449 ns | 1.26 µs | 1.76 µs | 2.62 µs |
| D616 | 115 ns | 561 ns | 1.75 µs | 2 µs | 4.2 µs |
| D924 | 116 ns | 1.58 µs | 2.99 µs | 5.02 µs | 6.08 µs |
| D1232 | 158 ns | 2.2 µs | 4.55 µs | 8.27 µs | 14.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,156.8 124.4,155.8 160.5,151.6 196.7,139.3 232.9,134.8 269.1,128.4 305.3,131.3 341.5,111.0 377.6,108.0 413.8,107.9 450.0,103.4 450.0,38.3 413.8,50.5 377.6,55.9 341.5,62.7 305.3,72.6 269.1,76.9 232.9,93.7 196.7,97.9 160.5,109.7 124.4,114.6 88.2,130.1 52.0,152.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,156.8 124.4,155.8 160.5,151.6 196.7,139.3 232.9,134.8 269.1,128.4 305.3,131.3 341.5,111.0 377.6,108.0 413.8,107.9 450.0,103.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.4 88.2,141.7 124.4,132.3 160.5,128.6 196.7,121.2 232.9,121.1 269.1,111.0 305.3,111.2 341.5,88.3 377.6,85.0 413.8,70.0 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.2 88.2,129.2 124.4,126.0 160.5,121.4 196.7,111.7 232.9,107.3 269.1,91.2 305.3,86.5 341.5,73.3 377.6,68.6 413.8,60.8 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,134.4 124.4,121.0 160.5,112.6 196.7,100.3 232.9,97.5 269.1,84.8 305.3,80.8 341.5,68.5 377.6,66.6 413.8,53.3 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.8 88.2,130.1 124.4,114.6 160.5,109.7 196.7,97.9 232.9,93.7 269.1,76.9 305.3,72.6 341.5,62.7 377.6,55.9 413.8,50.5 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.703 ns | 0.371 ns | 0.703 ns | 0.622 ns |
| D38 | 1.45 ns | 0.949 ns | 1.33 ns | 0.954 ns | 1.15 ns |
| D57 | 1.74 ns | 1.87 ns | 1.87 ns | 1.06 ns | 1.87 ns |
| D76 | 2.17 ns | 1.71 ns | 2.16 ns | 2.17 ns | 2.49 ns |
| D115 | 2.83 ns | 2.83 ns | 2.83 ns | 3.28 ns | 3.28 ns |
| D153 | 3.82 ns | 2.91 ns | 4.6 ns | 4.3 ns | 3.57 ns |
| D230 | 6.65 ns | 5.21 ns | 7.24 ns | 7.24 ns | 7.16 ns |
| D307 | 4.95 ns | 5.25 ns | 12.4 ns | 7.19 ns | 7.81 ns |
| D462 | 15.1 ns | 16.7 ns | 14.9 ns | 14.9 ns | 17 ns |
| D616 | 23 ns | 20.5 ns | 19.9 ns | 16.9 ns | 21.6 ns |
| D924 | 34.2 ns | 84.8 ns | 75.2 ns | 74.8 ns | 56.6 ns |
| D1232 | 41.8 ns | 62 ns | 44.7 ns | 64.9 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,132.6 124.4,127.3 160.5,121.0 196.7,113.2 232.9,104.5 269.1,88.5 305.3,97.0 341.5,64.8 377.6,52.5 413.8,41.1 450.0,35.3 450.0,20.4 413.8,26.5 377.6,54.3 341.5,61.3 305.3,83.8 269.1,86.3 232.9,106.5 196.7,108.9 160.5,117.0 124.4,125.2 88.2,139.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,132.6 124.4,127.3 160.5,121.0 196.7,113.2 232.9,104.5 269.1,88.5 305.3,97.0 341.5,64.8 377.6,52.5 413.8,41.1 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,144.8 124.4,125.2 160.5,127.8 196.7,113.2 232.9,112.4 269.1,95.5 305.3,95.4 341.5,61.9 377.6,55.9 413.8,14.8 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.0 88.2,135.1 124.4,125.2 160.5,121.0 196.7,113.2 232.9,99.1 269.1,86.0 305.3,70.5 341.5,65.1 377.6,56.7 413.8,18.3 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,144.7 124.4,141.7 160.5,120.9 196.7,108.9 232.9,101.1 269.1,86.0 305.3,86.2 341.5,65.1 377.6,61.5 413.8,18.4 450.0,22.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,139.2 124.4,125.2 160.5,117.0 196.7,108.9 232.9,106.5 269.1,86.3 305.3,83.8 341.5,61.3 377.6,54.3 413.8,26.5 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 1.97 ns | 2.46 ns | 2.18 ns |
| D38 | 3.52 ns | 2.12 ns | 4.16 ns | 1.99 ns | 3.62 ns |
| D57 | 8.09 ns | 7.16 ns | 7.17 ns | 3.49 ns | 7.16 ns |
| D76 | 9.51 ns | 6.28 ns | 9.51 ns | 9.55 ns | 8.43 ns |
| D115 | 12.9 ns | 12.7 ns | 12.7 ns | 12.7 ns | 12.7 ns |
| D153 | 16.2 ns | 12.3 ns | 20 ns | 16.1 ns | 15.6 ns |
| D230 | 36.5 ns | 23.6 ns | 36.2 ns | 42.9 ns | 32.1 ns |
| D307 | 23.7 ns | 22 ns | 62.8 ns | 29.7 ns | 29 ns |
| D462 | 74.5 ns | 86.9 ns | 73.2 ns | 72.5 ns | 82.9 ns |
| D616 | 105 ns | 71.9 ns | 86 ns | 70.5 ns | 92.3 ns |
| D924 | 72.1 ns | 109 ns | 97.3 ns | 96.7 ns | 70.4 ns |
| D1232 | 127 ns | 138 ns | 89.4 ns | 119 ns | 129 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,173.6 124.4,149.5 160.5,144.8 196.7,135.9 232.9,129.4 269.1,105.8 305.3,118.4 341.5,85.2 377.6,75.2 413.8,86.1 450.0,69.6 450.0,69.4 413.8,86.8 377.6,79.0 341.5,82.1 305.3,112.5 269.1,109.6 232.9,130.5 196.7,136.3 160.5,148.3 124.4,153.0 88.2,172.7 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,173.6 124.4,149.5 160.5,144.8 196.7,135.9 232.9,129.4 269.1,105.8 305.3,118.4 341.5,85.2 377.6,75.2 413.8,86.1 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,188.2 124.4,153.0 160.5,156.8 196.7,136.3 232.9,137.2 269.1,118.5 305.3,120.5 341.5,80.7 377.6,86.2 413.8,74.2 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.4 88.2,168.7 124.4,153.0 160.5,144.8 196.7,136.4 232.9,123.2 269.1,106.1 305.3,90.1 341.5,85.7 377.6,81.0 413.8,77.5 450.0,79.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,190.1 124.4,173.8 160.5,144.7 196.7,136.3 232.9,129.5 269.1,101.1 305.3,111.9 341.5,86.0 377.6,86.8 413.8,77.6 450.0,71.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,172.7 124.4,153.0 160.5,148.3 196.7,136.3 232.9,130.5 269.1,109.6 305.3,112.5 341.5,82.1 377.6,79.0 413.8,86.8 450.0,69.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 1.05 ns | 1.14 ns | 1.06 ns | 0.936 ns |
| D38 | 1.81 ns | 0.924 ns | 1.62 ns | 0.925 ns | 1.44 ns |
| D57 | 2.5 ns | 2.28 ns | 2.27 ns | 1.3 ns | 2.27 ns |
| D76 | 3.45 ns | 2.44 ns | 3.45 ns | 3.46 ns | 3.09 ns |
| D115 | 4.9 ns | 4.9 ns | 4.89 ns | 4.88 ns | 4.93 ns |
| D153 | 7.58 ns | 6.38 ns | 8.48 ns | 7.56 ns | 6.59 ns |
| D230 | 17.6 ns | 15.5 ns | 17.6 ns | 17.6 ns | 16.1 ns |
| D307 | 13.8 ns | 12.7 ns | 24.9 ns | 16.5 ns | 18.1 ns |
| D462 | 37.7 ns | 40.3 ns | 36.8 ns | 36.7 ns | 40.9 ns |
| D616 | 60.1 ns | 49.6 ns | 54.4 ns | 42.3 ns | 59.6 ns |
| D924 | 61.2 ns | 84.7 ns | 74.8 ns | 74.7 ns | 67.7 ns |
| D1232 | 83.6 ns | 95.3 ns | 78 ns | 95.9 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,147.1 124.4,140.1 160.5,133.1 196.7,125.5 232.9,116.0 269.1,97.7 305.3,103.0 341.5,81.2 377.6,71.1 413.8,70.7 450.0,63.9 450.0,58.7 413.8,68.5 377.6,71.2 341.5,79.4 305.3,97.1 269.1,99.6 232.9,119.1 196.7,125.4 160.5,135.5 124.4,142.2 88.2,152.1 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,147.1 124.4,140.1 160.5,133.1 196.7,125.5 232.9,116.0 269.1,97.7 305.3,103.0 341.5,81.2 377.6,71.1 413.8,70.7 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,161.7 124.4,142.1 160.5,140.6 196.7,125.5 232.9,119.8 269.1,100.5 305.3,104.8 341.5,79.7 377.6,75.2 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,149.5 124.4,142.2 160.5,133.1 196.7,125.5 232.9,113.6 269.1,97.7 305.3,90.2 341.5,81.7 377.6,73.2 413.8,66.3 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,161.7 124.4,154.4 160.5,133.1 196.7,125.6 232.9,116.1 269.1,97.7 305.3,99.1 341.5,81.8 377.6,78.7 413.8,66.3 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,152.1 124.4,142.2 160.5,135.5 196.7,125.4 232.9,119.1 269.1,99.6 305.3,97.1 341.5,79.4 377.6,71.2 413.8,68.5 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
