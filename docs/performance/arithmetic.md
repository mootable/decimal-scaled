# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.84 ns | 1.25 ns | 1.05 ns | 0.84 ns | 1.01 ns |
| D38 | 1.61 ns | 1.61 ns | 1.83 ns | 1.61 ns | 1.62 ns |
| D57 | 2.26 ns | 1.21 ns | 1.94 ns | 2.25 ns | 2.25 ns |
| D76 | 3.07 ns | 3.09 ns | 3.09 ns | 3.49 ns | 3.08 ns |
| D115 | 4.41 ns | 2.57 ns | 4.39 ns | 2.92 ns | 3.35 ns |
| D153 | 3.33 ns | 5.9 ns | 5.9 ns | 6.64 ns | 5.91 ns |
| D230 | 12.2 ns | 13.6 ns | 15.4 ns | 13.9 ns | 13.8 ns |
| D307 | 18.6 ns | 13.7 ns | 19.6 ns | 9.98 ns | 10 ns |
| D462 | 32.7 ns | 28.8 ns | 28.9 ns | 33.2 ns | 29.3 ns |
| D616 | 45 ns | 51.5 ns | 45.5 ns | 47.7 ns | 52.3 ns |
| D924 | 71.1 ns | 82.4 ns | 59.8 ns | 54 ns | 98.4 ns |
| D1232 | 95 ns | 95 ns | 83.2 ns | 94.5 ns | 71 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,148.4 88.2,129.6 124.4,119.8 160.5,110.9 196.7,100.3 232.9,108.5 269.1,71.0 305.3,58.7 341.5,42.3 377.6,33.1 413.8,19.9 450.0,11.5 450.0,19.9 413.8,10.5 377.6,28.8 341.5,45.6 305.3,76.7 269.1,67.3 232.9,91.9 196.7,108.3 160.5,110.7 124.4,119.9 88.2,129.4 52.0,143.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,148.4 88.2,129.6 124.4,119.8 160.5,110.9 196.7,100.3 232.9,108.5 269.1,71.0 305.3,58.7 341.5,42.3 377.6,33.1 413.8,19.9 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,137.0 88.2,129.5 124.4,137.8 160.5,110.7 196.7,116.0 232.9,91.9 269.1,67.8 305.3,67.6 341.5,46.0 377.6,29.2 413.8,15.6 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,125.8 124.4,124.1 160.5,110.7 196.7,100.5 232.9,92.0 269.1,64.2 305.3,57.2 341.5,45.9 377.6,32.8 413.8,24.9 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,148.4 88.2,129.6 124.4,119.9 160.5,107.2 196.7,112.3 232.9,88.5 269.1,67.1 305.3,76.7 341.5,41.9 377.6,31.4 413.8,27.9 450.0,11.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,143.0 88.2,129.4 124.4,119.9 160.5,110.7 196.7,108.3 232.9,91.9 269.1,67.3 305.3,76.7 341.5,45.6 377.6,28.8 413.8,10.5 450.0,19.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 5.43 ns | 7.03 ns | 6.02 ns | 8.65 ns |
| D38 | 9.84 ns | 11.7 ns | 14.3 ns | 60.2 ns | 62.6 ns |
| D57 | 23.4 ns | 18.5 ns | 57.3 ns | 106 ns | 109 ns |
| D76 | 25.9 ns | 60.9 ns | 76.6 ns | 109 ns | 134 ns |
| D115 | 42.7 ns | 50.1 ns | 104 ns | 139 ns | 216 ns |
| D153 | 30.8 ns | 109 ns | 143 ns | 254 ns | 307 ns |
| D230 | 64.6 ns | 117 ns | 253 ns | 387 ns | 565 ns |
| D307 | 118 ns | 153 ns | 396 ns | 351 ns | 558 ns |
| D462 | 230 ns | 419 ns | 671 ns | 1.14 µs | 1.38 µs |
| D616 | 229 ns | 647 ns | 1.01 µs | 1.82 µs | 1.92 µs |
| D924 | 332 ns | 1.1 µs | 1.47 µs | 1.98 µs | 4.79 µs |
| D1232 | 492 ns | 1.74 µs | 2.97 µs | 3.68 µs | 5.85 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.6 88.2,160.4 124.4,141.5 160.5,139.3 196.7,128.5 232.9,135.6 269.1,119.5 305.3,106.4 341.5,92.0 377.6,92.0 413.8,84.0 450.0,75.4 450.0,21.6 413.8,26.0 377.6,45.8 341.5,52.9 305.3,72.7 269.1,72.4 232.9,85.6 196.7,93.3 160.5,103.6 124.4,108.1 88.2,120.2 52.0,163.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.6 88.2,160.4 124.4,141.5 160.5,139.3 196.7,128.5 232.9,135.6 269.1,119.5 305.3,106.4 341.5,92.0 377.6,92.0 413.8,84.0 450.0,75.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.3 88.2,156.7 124.4,146.6 160.5,120.8 196.7,125.0 232.9,108.1 269.1,106.6 305.3,100.8 341.5,78.9 377.6,69.4 413.8,57.9 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.7 88.2,152.2 124.4,122.1 160.5,115.8 196.7,109.1 232.9,102.2 269.1,89.8 305.3,80.1 341.5,68.7 377.6,59.7 413.8,51.6 450.0,36.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.0 88.2,121.0 124.4,108.8 160.5,108.0 196.7,102.8 232.9,89.8 269.1,80.6 305.3,82.7 341.5,57.2 377.6,47.0 413.8,45.2 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.2 88.2,120.2 124.4,108.1 160.5,103.6 196.7,93.3 232.9,85.6 269.1,72.4 305.3,72.7 341.5,52.9 377.6,45.8 413.8,26.0 450.0,21.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.477 ns | 2.87 ns | 3.21 ns | 3.11 ns | 4.75 ns |
| D38 | 3.51 ns | 13.8 ns | 30.3 ns | 27.5 ns | 27.7 ns |
| D57 | 6.23 ns | 12.1 ns | 27.2 ns | 71.7 ns | 71.5 ns |
| D76 | 8.1 ns | 35.4 ns | 42.2 ns | 83.6 ns | 102 ns |
| D115 | 13.4 ns | 26.6 ns | 85.4 ns | 151 ns | 213 ns |
| D153 | 8.99 ns | 50.2 ns | 111 ns | 259 ns | 352 ns |
| D230 | 23.7 ns | 91.1 ns | 367 ns | 518 ns | 978 ns |
| D307 | 44.8 ns | 112 ns | 509 ns | 532 ns | 725 ns |
| D462 | 88 ns | 417 ns | 1.26 µs | 1.85 µs | 2.43 µs |
| D616 | 92.6 ns | 735 ns | 1.73 µs | 2.69 µs | 3.32 µs |
| D924 | 127 ns | 1.51 µs | 2.22 µs | 3.56 µs | 8.33 µs |
| D1232 | 189 ns | 2.23 µs | 3.98 µs | 7.03 µs | 10 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.4 88.2,158.5 124.4,150.2 160.5,146.4 196.7,139.1 232.9,144.9 269.1,130.8 305.3,121.6 341.5,111.8 377.6,111.1 413.8,106.5 450.0,100.8 450.0,43.3 413.8,46.0 377.6,59.3 341.5,63.8 305.3,81.3 269.1,77.0 232.9,91.8 196.7,99.0 160.5,109.7 124.4,114.9 88.2,128.6 52.0,154.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.4 88.2,158.5 124.4,150.2 160.5,146.4 196.7,139.1 232.9,144.9 269.1,130.8 305.3,121.6 341.5,111.8 377.6,111.1 413.8,106.5 450.0,100.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,138.7 124.4,140.6 160.5,125.0 196.7,129.2 232.9,120.0 269.1,111.4 305.3,108.3 341.5,89.3 377.6,81.1 413.8,70.7 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,127.3 124.4,128.8 160.5,122.5 196.7,112.3 232.9,108.5 269.1,91.2 305.3,86.5 341.5,73.3 377.6,68.7 413.8,65.1 450.0,56.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,128.7 124.4,114.8 160.5,112.6 196.7,104.0 232.9,96.2 269.1,86.2 305.3,85.8 341.5,67.7 377.6,62.3 413.8,58.3 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.1 88.2,128.6 124.4,114.9 160.5,109.7 196.7,99.0 232.9,91.8 269.1,77.0 305.3,81.3 341.5,63.8 377.6,59.3 413.8,46.0 450.0,43.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.271 ns | 0.622 ns | 0.703 ns | 0.271 ns | 0.662 ns |
| D38 | 1.41 ns | 1.42 ns | 1.45 ns | 1.41 ns | 1.42 ns |
| D57 | 1.68 ns | 0.886 ns | 1.35 ns | 1.68 ns | 1.68 ns |
| D76 | 2.09 ns | 2.1 ns | 2.09 ns | 2.16 ns | 2.49 ns |
| D115 | 2.83 ns | 1.94 ns | 2.83 ns | 2.3 ns | 2.78 ns |
| D153 | 1.99 ns | 3.79 ns | 4.29 ns | 4.61 ns | 4.29 ns |
| D230 | 4.3 ns | 4.87 ns | 7.43 ns | 7.18 ns | 7.18 ns |
| D307 | 10.9 ns | 6.53 ns | 12.5 ns | 5.04 ns | 5.02 ns |
| D462 | 18.3 ns | 14.9 ns | 15.4 ns | 17 ns | 15.3 ns |
| D616 | 18.8 ns | 22 ns | 20.1 ns | 24.8 ns | 24.6 ns |
| D924 | 55 ns | 82.6 ns | 55.1 ns | 49.6 ns | 96.1 ns |
| D1232 | 47.1 ns | 61.6 ns | 51.7 ns | 67.1 ns | 52.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,181.1 88.2,133.3 124.4,128.3 160.5,122.0 196.7,113.2 232.9,123.4 269.1,101.1 305.3,74.2 341.5,59.2 377.6,58.4 413.8,27.3 450.0,31.8 450.0,28.8 413.8,11.1 377.6,50.6 341.5,64.3 305.3,96.6 269.1,86.3 232.9,101.2 196.7,113.7 160.5,116.9 124.4,128.3 88.2,133.3 52.0,155.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,181.1 88.2,133.3 124.4,128.3 160.5,122.0 196.7,113.2 232.9,123.4 269.1,101.1 305.3,74.2 341.5,59.2 377.6,58.4 413.8,27.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,133.3 124.4,146.8 160.5,121.9 196.7,124.2 232.9,104.7 269.1,97.5 305.3,89.0 341.5,65.1 377.6,53.9 413.8,15.5 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,134.7 160.5,121.9 196.7,113.3 232.9,101.1 269.1,85.3 305.3,70.2 341.5,64.2 377.6,56.4 413.8,27.3 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.1 88.2,133.3 124.4,128.3 160.5,121.0 196.7,119.2 232.9,99.1 269.1,86.3 305.3,96.5 341.5,61.3 377.6,50.4 413.8,30.3 450.0,21.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.3 88.2,133.3 124.4,128.3 160.5,116.9 196.7,113.7 232.9,101.2 269.1,86.3 305.3,96.6 341.5,64.3 377.6,50.6 413.8,11.1 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.44 ns | 1.87 ns | 2.11 ns | 2.4 ns | 2.33 ns |
| D38 | 3.31 ns | 3.31 ns | 4.39 ns | 3.19 ns | 3.2 ns |
| D57 | 7.16 ns | 3.22 ns | 6.27 ns | 7.17 ns | 7.17 ns |
| D76 | 8.51 ns | 8.56 ns | 8.46 ns | 9.83 ns | 8.72 ns |
| D115 | 12.7 ns | 7.49 ns | 12.8 ns | 7.64 ns | 9.28 ns |
| D153 | 9.24 ns | 16.1 ns | 16 ns | 20.1 ns | 17.5 ns |
| D230 | 24.4 ns | 22.7 ns | 36.3 ns | 32.2 ns | 31.8 ns |
| D307 | 41.9 ns | 27.6 ns | 48 ns | 20.6 ns | 19.4 ns |
| D462 | 88.9 ns | 73.5 ns | 73.9 ns | 88.3 ns | 71.3 ns |
| D616 | 84.8 ns | 105 ns | 78.9 ns | 85.1 ns | 87.2 ns |
| D924 | 115 ns | 108 ns | 83 ns | 80.7 ns | 117 ns |
| D1232 | 133 ns | 133 ns | 110 ns | 107 ns | 77 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.4 88.2,175.4 124.4,153.0 160.5,148.0 196.7,136.3 232.9,145.6 269.1,117.5 305.3,101.9 341.5,80.1 377.6,81.4 413.8,72.6 450.0,68.4 450.0,84.2 413.8,72.0 377.6,80.6 341.5,86.5 305.3,124.2 269.1,109.8 232.9,127.1 196.7,145.5 160.5,147.3 124.4,153.0 88.2,176.3 52.0,185.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.4 88.2,175.4 124.4,153.0 160.5,148.0 196.7,136.3 232.9,145.6 269.1,117.5 305.3,101.9 341.5,80.1 377.6,81.4 413.8,72.6 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.3 124.4,176.2 160.5,147.8 196.7,151.7 232.9,129.5 269.1,119.5 305.3,113.9 341.5,85.6 377.6,75.4 413.8,74.4 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,167.2 124.4,156.8 160.5,148.2 196.7,136.2 232.9,129.7 269.1,106.0 305.3,97.9 341.5,85.4 377.6,83.5 413.8,82.1 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.7 88.2,176.4 124.4,153.0 160.5,143.8 196.7,151.1 232.9,123.2 269.1,109.4 305.3,122.5 341.5,80.3 377.6,81.3 413.8,82.9 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.6 88.2,176.3 124.4,153.0 160.5,147.3 196.7,145.5 232.9,127.1 269.1,109.8 305.3,124.2 341.5,86.5 377.6,80.6 413.8,72.0 450.0,84.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.865 ns | 0.937 ns | 1.05 ns | 0.863 ns | 1.01 ns |
| D38 | 1.6 ns | 1.6 ns | 1.82 ns | 1.6 ns | 1.6 ns |
| D57 | 2.25 ns | 1.27 ns | 1.95 ns | 2.25 ns | 2.26 ns |
| D76 | 3.09 ns | 3.08 ns | 3.08 ns | 3.46 ns | 3.08 ns |
| D115 | 4.88 ns | 3.14 ns | 4.88 ns | 3.69 ns | 4.03 ns |
| D153 | 4.61 ns | 7.64 ns | 7.63 ns | 8.46 ns | 7.68 ns |
| D230 | 13.7 ns | 14.9 ns | 17.6 ns | 16.2 ns | 16.2 ns |
| D307 | 23.4 ns | 15.9 ns | 25.1 ns | 12.7 ns | 12.6 ns |
| D462 | 40.6 ns | 37 ns | 37.4 ns | 43.3 ns | 37.4 ns |
| D616 | 45.3 ns | 55.4 ns | 45 ns | 49.7 ns | 53.7 ns |
| D924 | 71.1 ns | 83.4 ns | 66.2 ns | 59.8 ns | 97.7 ns |
| D1232 | 95.5 ns | 95.7 ns | 83.5 ns | 94.9 ns | 79.3 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,147.5 88.2,129.7 124.4,119.8 160.5,110.7 196.7,97.5 232.9,99.1 269.1,67.5 305.3,52.1 341.5,36.1 377.6,32.9 413.8,19.9 450.0,11.3 450.0,16.7 413.8,10.7 377.6,28.0 341.5,38.5 305.3,69.9 269.1,62.8 232.9,84.3 196.7,103.0 160.5,110.7 124.4,119.8 88.2,129.7 52.0,142.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,147.5 88.2,129.7 124.4,119.8 160.5,110.7 196.7,97.5 232.9,99.1 269.1,67.5 305.3,52.1 341.5,36.1 377.6,32.9 413.8,19.9 450.0,11.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.2 88.2,129.7 124.4,136.5 160.5,110.7 196.7,110.2 232.9,84.4 269.1,65.0 305.3,63.3 341.5,38.8 377.6,27.1 413.8,15.2 450.0,11.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,125.9 124.4,124.0 160.5,110.7 196.7,97.4 232.9,84.5 269.1,60.2 305.3,50.0 341.5,38.5 377.6,33.1 413.8,22.0 450.0,15.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.6 88.2,129.7 124.4,119.8 160.5,107.4 196.7,105.5 232.9,81.5 269.1,62.8 305.3,69.8 341.5,34.2 377.6,30.2 413.8,24.9 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,142.9 88.2,129.7 124.4,119.8 160.5,110.7 196.7,103.0 232.9,84.3 269.1,62.8 305.3,69.9 341.5,38.5 377.6,28.0 413.8,10.7 450.0,16.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
