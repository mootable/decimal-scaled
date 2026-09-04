# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.937 ns | 0.935 ns | 1.05 ns | 1.06 ns | 1.01 ns |
| D38 | 0.991 ns | 1.61 ns | 1.83 ns | 1.62 ns | 1.83 ns |
| D57 | 2.28 ns | 1.39 ns | 2.28 ns | 2.28 ns | 2.28 ns |
| D76 | 3.09 ns | 2.7 ns | 3.48 ns | 2.71 ns | 1.79 ns |
| D115 | 3.87 ns | 3.88 ns | 2.31 ns | 3.33 ns | 4.41 ns |
| D153 | 6.65 ns | 5.9 ns | 6.63 ns | 6.63 ns | 5.9 ns |
| D230 | 15.4 ns | 13.9 ns | 15.4 ns | 15.4 ns | 13.9 ns |
| D307 | 18.5 ns | 11.8 ns | 12 ns | 18.6 ns | 18.6 ns |
| D462 | 29.8 ns | 29 ns | 28.9 ns | 28.9 ns | 29.4 ns |
| D616 | 49.9 ns | 54.3 ns | 49.8 ns | 34.2 ns | 45.3 ns |
| D924 | 74.6 ns | 74.7 ns | 74.5 ns | 86.3 ns | 79.8 ns |
| D1232 | 94.9 ns | 107 ns | 80.1 ns | 106 ns | 104 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,160.2 124.4,142.1 160.5,135.5 196.7,130.6 232.9,118.9 269.1,100.7 305.3,96.6 341.5,86.3 377.6,75.1 413.8,66.4 450.0,61.1 450.0,59.2 413.8,64.9 377.6,77.2 341.5,86.6 305.3,96.5 269.1,102.9 232.9,121.5 196.7,127.8 160.5,147.3 124.4,142.1 88.2,146.8 52.0,159.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,160.2 124.4,142.1 160.5,135.5 196.7,130.6 232.9,118.9 269.1,100.7 305.3,96.6 341.5,86.3 377.6,75.1 413.8,66.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.6 124.4,152.8 160.5,138.5 196.7,130.6 232.9,121.5 269.1,102.9 305.3,106.5 341.5,86.9 377.6,73.3 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.1 160.5,132.9 196.7,141.9 232.9,118.9 269.1,100.7 305.3,106.1 341.5,87.0 377.6,75.2 413.8,66.4 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.1 160.5,138.4 196.7,133.9 232.9,118.9 269.1,100.7 305.3,96.5 341.5,86.9 377.6,83.3 413.8,63.2 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.7 88.2,146.8 124.4,142.1 160.5,147.3 196.7,127.8 232.9,121.5 269.1,102.9 305.3,96.5 341.5,86.6 377.6,77.2 413.8,64.9 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.96 ns | 5.92 ns | 7.03 ns | 8.79 ns | 7.24 ns |
| D38 | 6.23 ns | 11.4 ns | 14.3 ns | 59.5 ns | 68.3 ns |
| D57 | 23.4 ns | 19.8 ns | 68.9 ns | 111 ns | 113 ns |
| D76 | 26.4 ns | 51.8 ns | 83.7 ns | 99.8 ns | 107 ns |
| D115 | 36.6 ns | 68.7 ns | 59.6 ns | 159 ns | 230 ns |
| D153 | 57.7 ns | 110 ns | 155 ns | 263 ns | 321 ns |
| D230 | 93.9 ns | 155 ns | 252 ns | 437 ns | 569 ns |
| D307 | 117 ns | 129 ns | 244 ns | 604 ns | 890 ns |
| D462 | 195 ns | 426 ns | 694 ns | 1.07 µs | 1.23 µs |
| D616 | 228 ns | 600 ns | 890 ns | 1.19 µs | 2.15 µs |
| D924 | 372 ns | 1.08 µs | 2.07 µs | 2.83 µs | 4.27 µs |
| D1232 | 503 ns | 2 µs | 2.51 µs | 4.71 µs | 6.99 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.2 88.2,170.3 124.4,141.5 160.5,138.9 196.7,131.9 232.9,121.9 269.1,111.4 305.3,106.6 341.5,95.5 377.6,92.1 413.8,81.5 450.0,74.9 450.0,17.8 413.8,28.5 377.6,43.4 341.5,55.4 305.3,62.5 269.1,72.2 232.9,84.7 196.7,91.9 160.5,108.6 124.4,107.4 88.2,118.3 52.0,167.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,170.3 124.4,141.5 160.5,138.9 196.7,131.9 232.9,121.9 269.1,111.4 305.3,106.6 341.5,95.5 377.6,92.1 413.8,81.5 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,157.1 124.4,145.2 160.5,124.3 196.7,118.1 232.9,108.0 269.1,100.6 305.3,104.4 341.5,78.5 377.6,71.1 413.8,58.3 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.7 88.2,152.2 124.4,118.1 160.5,113.9 196.7,121.2 232.9,100.5 269.1,89.9 305.3,90.6 341.5,67.9 377.6,62.5 413.8,44.2 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.8 88.2,121.3 124.4,107.6 160.5,110.0 196.7,99.9 232.9,89.0 269.1,78.0 305.3,71.0 341.5,58.5 377.6,56.2 413.8,37.4 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.0 88.2,118.3 124.4,107.4 160.5,108.6 196.7,91.9 232.9,84.7 269.1,72.2 305.3,62.5 341.5,55.4 377.6,43.4 413.8,28.5 450.0,17.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.87 ns | 3.2 ns | 5.03 ns | 3.78 ns |
| D38 | 2.34 ns | 15 ns | 28.3 ns | 27.1 ns | 30.6 ns |
| D57 | 6.55 ns | 12.8 ns | 32.8 ns | 71.5 ns | 71.6 ns |
| D76 | 7.79 ns | 29.6 ns | 46.1 ns | 66.6 ns | 72.9 ns |
| D115 | 10.5 ns | 42.5 ns | 46.1 ns | 166 ns | 231 ns |
| D153 | 16.8 ns | 52.7 ns | 118 ns | 259 ns | 355 ns |
| D230 | 28 ns | 116 ns | 368 ns | 567 ns | 995 ns |
| D307 | 44.5 ns | 94.4 ns | 292 ns | 1.03 µs | 1.39 µs |
| D462 | 79.2 ns | 421 ns | 1.28 µs | 1.78 µs | 2.21 µs |
| D616 | 91 ns | 649 ns | 1.48 µs | 1.76 µs | 3.91 µs |
| D924 | 138 ns | 1.49 µs | 2.97 µs | 5.45 µs | 7.53 µs |
| D1232 | 190 ns | 2.37 µs | 3.74 µs | 8.96 µs | 13.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,164.4 124.4,149.5 160.5,146.9 196.7,142.6 232.9,135.8 269.1,128.4 305.3,121.7 341.5,113.4 377.6,111.4 413.8,105.3 450.0,100.7 450.0,39.4 413.8,47.4 377.6,56.9 341.5,65.2 305.3,71.9 269.1,76.7 232.9,91.6 196.7,97.9 160.5,114.6 124.4,114.8 88.2,127.1 52.0,157.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,164.4 124.4,149.5 160.5,146.9 196.7,142.6 232.9,135.8 269.1,128.4 305.3,121.7 341.5,113.4 377.6,111.4 413.8,105.3 450.0,100.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,137.5 124.4,139.7 160.5,127.6 196.7,122.4 232.9,119.3 269.1,107.8 305.3,110.8 341.5,89.2 377.6,82.9 413.8,70.9 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,128.3 124.4,126.1 160.5,121.2 196.7,121.2 232.9,107.6 269.1,91.1 305.3,94.5 341.5,73.1 377.6,71.0 413.8,60.9 450.0,57.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,128.9 124.4,114.8 160.5,115.9 196.7,102.7 232.9,96.2 269.1,84.9 305.3,76.2 341.5,68.3 377.6,68.5 413.8,52.1 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.4 88.2,127.1 124.4,114.8 160.5,114.6 196.7,97.9 232.9,91.6 269.1,76.7 305.3,71.9 341.5,65.2 377.6,56.9 413.8,47.4 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.622 ns | 0.703 ns | 0.703 ns | 0.319 ns |
| D38 | 0.803 ns | 1.32 ns | 1.45 ns | 1.33 ns | 1.45 ns |
| D57 | 1.68 ns | 1.05 ns | 1.68 ns | 1.68 ns | 1.68 ns |
| D76 | 2.1 ns | 1.68 ns | 2.17 ns | 1.67 ns | 1.73 ns |
| D115 | 2.46 ns | 2.46 ns | 1.45 ns | 2.65 ns | 3.28 ns |
| D153 | 4.22 ns | 3.79 ns | 4.6 ns | 4.61 ns | 4.49 ns |
| D230 | 6.65 ns | 6 ns | 7.24 ns | 7.23 ns | 7.18 ns |
| D307 | 10.9 ns | 5.49 ns | 5.63 ns | 11.1 ns | 11.1 ns |
| D462 | 15 ns | 14.9 ns | 14.9 ns | 14.9 ns | 14.9 ns |
| D616 | 18 ns | 20.1 ns | 17.4 ns | 15.4 ns | 19.9 ns |
| D924 | 54.9 ns | 75.5 ns | 75.6 ns | 84.7 ns | 76.3 ns |
| D1232 | 47.1 ns | 69.8 ns | 54.9 ns | 69.7 ns | 65.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,149.7 124.4,128.3 160.5,121.9 196.7,117.3 232.9,101.6 269.1,88.5 305.3,74.2 341.5,64.9 377.6,59.6 413.8,27.3 450.0,31.8 450.0,22.2 413.8,17.8 377.6,56.7 341.5,65.1 305.3,73.6 269.1,86.3 232.9,99.9 196.7,108.9 160.5,127.5 124.4,128.3 88.2,132.6 52.0,176.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,149.7 124.4,128.3 160.5,121.9 196.7,117.3 232.9,101.6 269.1,88.5 305.3,74.2 341.5,64.9 377.6,59.6 413.8,27.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.2 124.4,142.0 160.5,128.3 196.7,117.3 232.9,104.8 269.1,91.5 305.3,94.1 341.5,65.1 377.6,56.5 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,128.3 160.5,120.9 196.7,132.5 232.9,99.2 269.1,86.0 305.3,93.3 341.5,65.1 377.6,60.7 413.8,18.1 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,128.3 160.5,128.4 196.7,115.1 232.9,99.1 269.1,86.0 305.3,73.7 341.5,65.1 377.6,64.1 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,176.4 88.2,132.6 124.4,128.3 160.5,127.5 196.7,108.9 232.9,99.9 269.1,86.3 305.3,73.6 341.5,65.1 377.6,56.7 413.8,17.8 450.0,22.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 2.11 ns | 2.46 ns | 2.8 ns |
| D38 | 1.92 ns | 3.24 ns | 3.51 ns | 3.17 ns | 3.53 ns |
| D57 | 7.17 ns | 3.67 ns | 7.16 ns | 7.16 ns | 7.17 ns |
| D76 | 8.43 ns | 7.45 ns | 9.83 ns | 7.63 ns | 4.81 ns |
| D115 | 11.2 ns | 11.2 ns | 6.23 ns | 8.65 ns | 12.8 ns |
| D153 | 20.1 ns | 16 ns | 20 ns | 20 ns | 17.3 ns |
| D230 | 36.6 ns | 32.3 ns | 36.3 ns | 36.2 ns | 32.1 ns |
| D307 | 41.7 ns | 23.4 ns | 24.7 ns | 42.6 ns | 42.6 ns |
| D462 | 75.9 ns | 72.3 ns | 74.1 ns | 73.3 ns | 70.5 ns |
| D616 | 77.4 ns | 81.9 ns | 90.5 ns | 54.3 ns | 77.9 ns |
| D924 | 100 ns | 100 ns | 92.8 ns | 105 ns | 90.3 ns |
| D1232 | 133 ns | 136 ns | 97 ns | 124 ns | 120 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,191.2 124.4,153.0 160.5,148.3 196.7,140.1 232.9,123.2 269.1,105.8 305.3,102.0 341.5,84.6 377.6,84.1 413.8,76.7 450.0,68.5 450.0,71.3 413.8,79.6 377.6,83.9 341.5,86.8 305.3,101.4 269.1,109.6 232.9,127.5 196.7,136.3 160.5,164.5 124.4,153.0 88.2,173.5 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,191.2 124.4,153.0 160.5,148.3 196.7,140.1 232.9,123.2 269.1,105.8 305.3,102.0 341.5,84.6 377.6,84.1 413.8,76.7 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.9 124.4,172.3 160.5,151.8 196.7,140.1 232.9,129.7 269.1,109.4 305.3,118.7 341.5,86.1 377.6,82.4 413.8,76.7 450.0,67.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,173.6 124.4,153.0 160.5,143.8 196.7,157.0 232.9,123.2 269.1,106.0 305.3,117.1 341.5,85.4 377.6,79.6 413.8,78.8 450.0,77.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,176.6 124.4,153.0 160.5,151.2 196.7,147.5 232.9,123.2 269.1,106.1 305.3,101.4 341.5,85.7 377.6,94.3 413.8,75.3 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,173.5 124.4,153.0 160.5,164.5 196.7,136.3 232.9,127.5 269.1,109.6 305.3,101.4 341.5,86.8 377.6,83.9 413.8,79.6 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.24 ns | 1.24 ns | 1.06 ns | 1.06 ns | 0.988 ns |
| D38 | 0.784 ns | 1.6 ns | 1.81 ns | 1.61 ns | 1.81 ns |
| D57 | 2.26 ns | 1.3 ns | 2.25 ns | 2.25 ns | 2.25 ns |
| D76 | 3.08 ns | 2.68 ns | 3.46 ns | 2.67 ns | 2.09 ns |
| D115 | 4.3 ns | 4.31 ns | 2.88 ns | 4.19 ns | 4.91 ns |
| D153 | 8.43 ns | 7.64 ns | 8.44 ns | 8.5 ns | 7.65 ns |
| D230 | 17.6 ns | 16.1 ns | 17.7 ns | 17.7 ns | 16.1 ns |
| D307 | 23.4 ns | 13.4 ns | 13.4 ns | 23.3 ns | 23.5 ns |
| D462 | 37.3 ns | 37 ns | 36.9 ns | 36.9 ns | 37.2 ns |
| D616 | 50 ns | 54 ns | 50 ns | 37.6 ns | 45.7 ns |
| D924 | 75.2 ns | 75.4 ns | 75 ns | 85.3 ns | 78.7 ns |
| D1232 | 95.5 ns | 107 ns | 89.1 ns | 106 ns | 104 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,155.3 88.2,165.3 124.4,142.3 160.5,135.5 196.7,128.3 232.9,113.7 269.1,97.7 305.3,91.6 341.5,81.4 377.6,75.0 413.8,66.2 450.0,61.0 450.0,59.2 413.8,65.2 377.6,77.0 341.5,81.5 305.3,91.4 269.1,99.6 232.9,115.8 196.7,125.5 160.5,144.0 124.4,142.4 88.2,147.1 52.0,160.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,155.3 88.2,165.3 124.4,142.3 160.5,135.5 196.7,128.3 232.9,113.7 269.1,97.7 305.3,91.6 341.5,81.4 377.6,75.0 413.8,66.2 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.3 88.2,149.7 124.4,154.2 160.5,138.6 196.7,128.3 232.9,115.8 269.1,99.7 305.3,103.6 341.5,81.6 377.6,73.4 413.8,66.1 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,142.4 160.5,133.1 196.7,137.1 232.9,113.7 269.1,97.7 305.3,103.6 341.5,81.6 377.6,75.0 413.8,66.2 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.4 160.5,138.7 196.7,128.9 232.9,113.5 269.1,97.7 305.3,91.6 341.5,81.6 377.6,81.2 413.8,63.5 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,147.1 124.4,142.4 160.5,144.0 196.7,125.5 232.9,115.8 269.1,99.6 305.3,91.4 341.5,81.5 377.6,77.0 413.8,65.2 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
