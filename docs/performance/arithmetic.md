# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.938 ns | 1.05 ns | 1.05 ns | 1.06 ns | 0.919 ns |
| D38 | 1.61 ns | 1.82 ns | 1.62 ns | 1.17 ns | 1.82 ns |
| D57 | 2.5 ns | 2.5 ns | 2.28 ns | 2.29 ns | 2.28 ns |
| D76 | 3.09 ns | 3.51 ns | 3.5 ns | 3.1 ns | 1.94 ns |
| D115 | 4.41 ns | 4.39 ns | 4.41 ns | 5 ns | 4.4 ns |
| D153 | 6.63 ns | 5.89 ns | 5.14 ns | 6.64 ns | 6.65 ns |
| D230 | 15.4 ns | 15.4 ns | 13.9 ns | 11.5 ns | 13.6 ns |
| D307 | 18.6 ns | 18.6 ns | 16.2 ns | 18.6 ns | 19.6 ns |
| D462 | 29.5 ns | 33.1 ns | 36 ns | 29.2 ns | 32.6 ns |
| D616 | 75.8 ns | 45.4 ns | 32.5 ns | 45.1 ns | 45.5 ns |
| D924 | 74.9 ns | 84.8 ns | 90.4 ns | 84.9 ns | 74.7 ns |
| D1232 | 106 ns | 93.7 ns | 106 ns | 95 ns | 94.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.6 124.4,140.1 160.5,135.5 196.7,127.8 232.9,118.9 269.1,100.7 305.3,96.6 341.5,86.5 377.6,66.0 413.8,66.3 450.0,58.7 450.0,61.1 413.8,66.3 377.6,77.1 341.5,84.3 305.3,95.4 269.1,103.3 232.9,118.8 196.7,127.8 160.5,145.6 124.4,142.1 88.2,147.0 52.0,161.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.6 124.4,140.1 160.5,135.5 196.7,127.8 232.9,118.9 269.1,100.7 305.3,96.6 341.5,86.5 377.6,66.0 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,132.8 196.7,127.9 232.9,121.5 269.1,100.7 305.3,96.6 341.5,84.0 377.6,77.1 413.8,63.6 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,142.1 160.5,132.8 196.7,127.8 232.9,124.4 269.1,102.8 305.3,99.6 341.5,82.2 377.6,84.4 413.8,62.2 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,156.6 124.4,142.0 160.5,135.4 196.7,125.0 232.9,118.9 269.1,106.9 305.3,96.6 341.5,86.7 377.6,77.3 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,147.0 124.4,142.1 160.5,145.6 196.7,127.8 232.9,118.8 269.1,103.3 305.3,95.4 341.5,84.3 377.6,77.1 413.8,66.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.78 ns | 6.21 ns | 6.19 ns | 8.79 ns | 6.02 ns |
| D38 | 9.88 ns | 12.6 ns | 15.6 ns | 71.2 ns | 67.2 ns |
| D57 | 35.4 ns | 51.4 ns | 68.7 ns | 106 ns | 107 ns |
| D76 | 39.6 ns | 66.8 ns | 85.1 ns | 108 ns | 109 ns |
| D115 | 56.5 ns | 82.9 ns | 105 ns | 193 ns | 223 ns |
| D153 | 69.8 ns | 108 ns | 122 ns | 252 ns | 334 ns |
| D230 | 114 ns | 167 ns | 228 ns | 295 ns | 510 ns |
| D307 | 132 ns | 223 ns | 311 ns | 575 ns | 949 ns |
| D462 | 217 ns | 436 ns | 688 ns | 1.06 µs | 1.47 µs |
| D616 | 274 ns | 607 ns | 672 ns | 1.82 µs | 2.26 µs |
| D924 | 398 ns | 1.22 µs | 2.3 µs | 2.84 µs | 4.54 µs |
| D1232 | 616 ns | 1.54 µs | 3.8 µs | 4.37 µs | 7.38 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.9 88.2,160.3 124.4,132.6 160.5,130.1 196.7,122.4 232.9,117.8 269.1,107.1 305.3,103.9 341.5,93.2 377.6,88.1 413.8,80.0 450.0,70.5 450.0,16.6 413.8,27.2 377.6,42.3 341.5,51.6 305.3,61.1 269.1,74.6 232.9,83.8 196.7,92.6 160.5,108.1 124.4,108.6 88.2,118.6 52.0,171.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.9 88.2,160.3 124.4,132.6 160.5,130.1 196.7,122.4 232.9,117.8 269.1,107.1 305.3,103.9 341.5,93.2 377.6,88.1 413.8,80.0 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,154.9 124.4,124.5 160.5,118.8 196.7,114.1 232.9,108.3 269.1,98.9 305.3,92.6 341.5,78.0 377.6,70.8 413.8,55.7 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,150.3 124.4,118.1 160.5,113.5 196.7,108.9 232.9,105.8 269.1,92.1 305.3,85.4 341.5,68.1 377.6,68.6 413.8,41.9 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.8 88.2,117.4 124.4,108.7 160.5,108.3 196.7,95.7 232.9,89.9 269.1,86.5 305.3,72.0 341.5,58.7 377.6,47.0 413.8,37.3 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.0 88.2,118.6 124.4,108.6 160.5,108.1 196.7,92.6 232.9,83.8 269.1,74.6 305.3,61.1 341.5,51.6 377.6,42.3 413.8,27.2 450.0,16.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.886 ns | 3.07 ns | 3.2 ns | 5.02 ns | 3.78 ns |
| D38 | 3.51 ns | 13.7 ns | 26.2 ns | 21.2 ns | 39 ns |
| D57 | 4.23 ns | 22 ns | 34.2 ns | 71.7 ns | 71.8 ns |
| D76 | 7.79 ns | 37.8 ns | 45.7 ns | 79.3 ns | 78.8 ns |
| D115 | 13.2 ns | 47.5 ns | 87.6 ns | 214 ns | 229 ns |
| D153 | 16.9 ns | 50 ns | 93.4 ns | 259 ns | 399 ns |
| D230 | 27.7 ns | 127 ns | 341 ns | 396 ns | 779 ns |
| D307 | 44.8 ns | 166 ns | 397 ns | 1.02 µs | 1.45 µs |
| D462 | 72.9 ns | 449 ns | 1.11 µs | 1.73 µs | 2.6 µs |
| D616 | 107 ns | 657 ns | 969 ns | 2.67 µs | 3.86 µs |
| D924 | 136 ns | 1.57 µs | 3.16 µs | 5.35 µs | 7.66 µs |
| D1232 | 204 ns | 1.94 µs | 5 µs | 8.17 µs | 12.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,147.0 196.7,139.3 232.9,135.8 269.1,128.6 305.3,121.6 341.5,114.6 377.6,109.0 413.8,105.5 450.0,99.7 450.0,39.6 413.8,47.2 377.6,57.1 341.5,62.9 305.3,71.2 269.1,80.3 232.9,90.0 196.7,98.0 160.5,113.4 124.4,114.8 88.2,123.6 52.0,157.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,147.0 196.7,139.3 232.9,135.8 269.1,128.6 305.3,121.6 341.5,114.6 377.6,109.0 413.8,105.5 450.0,99.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.4 88.2,138.8 124.4,131.9 160.5,124.1 196.7,120.8 232.9,120.0 269.1,106.5 305.3,102.7 341.5,88.2 377.6,82.7 413.8,70.1 450.0,67.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,129.4 124.4,125.5 160.5,121.3 196.7,111.9 232.9,111.0 269.1,92.2 305.3,90.0 341.5,75.2 377.6,77.1 413.8,60.0 450.0,53.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,132.5 124.4,114.8 160.5,113.4 196.7,99.0 232.9,96.2 269.1,90.1 305.3,76.4 341.5,68.7 377.6,62.4 413.8,52.4 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.4 88.2,123.6 124.4,114.8 160.5,113.4 196.7,98.0 232.9,90.0 269.1,80.3 305.3,71.2 341.5,62.9 377.6,57.1 413.8,47.2 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.703 ns | 0.703 ns | 0.703 ns | 0.297 ns |
| D38 | 1.32 ns | 1.45 ns | 1.32 ns | 1.09 ns | 1.45 ns |
| D57 | 1.74 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.87 ns |
| D76 | 2.19 ns | 2.17 ns | 2.17 ns | 2.19 ns | 1.76 ns |
| D115 | 2.85 ns | 2.85 ns | 2.85 ns | 3.55 ns | 3.28 ns |
| D153 | 4.22 ns | 3.79 ns | 3.57 ns | 4.61 ns | 4.6 ns |
| D230 | 6.65 ns | 6.65 ns | 7.16 ns | 4.42 ns | 5.25 ns |
| D307 | 10.9 ns | 11.1 ns | 7.96 ns | 11.1 ns | 12.5 ns |
| D462 | 15 ns | 17 ns | 17.2 ns | 14.9 ns | 16.7 ns |
| D616 | 23.6 ns | 19.9 ns | 13.7 ns | 19.7 ns | 19.9 ns |
| D924 | 54.9 ns | 84.6 ns | 85.1 ns | 84.7 ns | 76 ns |
| D1232 | 54.6 ns | 64.4 ns | 69.8 ns | 61.5 ns | 61.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,120.7 196.7,113.0 232.9,101.6 269.1,88.5 305.3,74.2 341.5,64.8 377.6,51.8 413.8,27.3 450.0,27.5 450.0,24.1 413.8,18.0 377.6,56.7 341.5,61.9 305.3,70.2 269.1,95.3 232.9,99.2 196.7,108.9 160.5,126.9 124.4,125.2 88.2,132.6 52.0,178.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,120.7 196.7,113.0 232.9,101.6 269.1,88.5 305.3,74.2 341.5,64.8 377.6,51.8 413.8,27.3 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,121.0 196.7,113.0 232.9,104.8 269.1,88.5 305.3,73.7 341.5,61.3 377.6,56.7 413.8,14.8 450.0,22.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,120.9 196.7,113.0 232.9,106.5 269.1,86.3 305.3,83.3 341.5,60.9 377.6,67.6 413.8,14.7 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,140.7 124.4,125.2 160.5,120.7 196.7,106.7 232.9,99.1 269.1,100.3 305.3,73.7 341.5,65.1 377.6,57.0 413.8,14.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.5 88.2,132.6 124.4,125.2 160.5,126.9 196.7,108.9 232.9,99.2 269.1,95.3 305.3,70.2 341.5,61.9 377.6,56.7 413.8,18.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 2.11 ns | 2.46 ns | 2.4 ns |
| D38 | 7.14 ns | 8.19 ns | 12 ns | 7.98 ns | 16.4 ns |
| D57 | 8.09 ns | 8.08 ns | 7.17 ns | 7.16 ns | 7.17 ns |
| D76 | 8.72 ns | 9.58 ns | 9.62 ns | 8.63 ns | 4.99 ns |
| D115 | 12.7 ns | 12.4 ns | 12.4 ns | 14.1 ns | 12.5 ns |
| D153 | 20.8 ns | 15.9 ns | 15.6 ns | 20 ns | 20 ns |
| D230 | 37.1 ns | 36.3 ns | 32.2 ns | 19.6 ns | 22.3 ns |
| D307 | 42 ns | 43.2 ns | 32.8 ns | 42.6 ns | 47.8 ns |
| D462 | 74 ns | 87.1 ns | 67.1 ns | 77 ns | 83.1 ns |
| D616 | 101 ns | 89.9 ns | 45.2 ns | 77.2 ns | 77.2 ns |
| D924 | 111 ns | 112 ns | 124 ns | 99.8 ns | 90 ns |
| D1232 | 157 ns | 123 ns | 131 ns | 114 ns | 116 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,153.1 124.4,149.5 160.5,147.3 196.7,136.4 232.9,122.2 269.1,105.4 305.3,101.8 341.5,85.4 377.6,76.3 413.8,73.6 450.0,63.7 450.0,72.5 413.8,79.7 377.6,84.1 341.5,82.0 305.3,98.0 269.1,120.1 232.9,123.2 196.7,136.9 160.5,163.5 124.4,153.0 88.2,129.0 52.0,184.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,153.1 124.4,149.5 160.5,147.3 196.7,136.4 232.9,122.2 269.1,105.4 305.3,101.8 341.5,85.4 377.6,76.3 413.8,73.6 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,149.1 124.4,149.5 160.5,144.6 196.7,137.1 232.9,130.0 269.1,106.0 305.3,101.0 341.5,80.7 377.6,79.7 413.8,73.3 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,137.9 124.4,153.0 160.5,144.5 196.7,137.0 232.9,130.5 269.1,109.5 305.3,108.9 341.5,88.2 377.6,99.7 413.8,70.5 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,149.9 124.4,153.0 160.5,147.6 196.7,133.5 232.9,123.2 269.1,123.9 305.3,101.3 341.5,84.2 377.6,84.2 413.8,76.7 450.0,72.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.7 88.2,129.0 124.4,153.0 160.5,163.5 196.7,136.9 232.9,123.2 269.1,120.1 305.3,98.0 341.5,82.0 377.6,84.1 413.8,79.7 450.0,72.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.934 ns | 1.06 ns | 1.06 ns | 1.05 ns | 0.866 ns |
| D38 | 1.61 ns | 1.81 ns | 1.6 ns | 1.06 ns | 1.81 ns |
| D57 | 2.5 ns | 2.5 ns | 2.27 ns | 2.27 ns | 2.28 ns |
| D76 | 3.1 ns | 3.46 ns | 3.45 ns | 3.1 ns | 2.23 ns |
| D115 | 4.85 ns | 4.83 ns | 4.84 ns | 5.55 ns | 4.88 ns |
| D153 | 8.45 ns | 7.7 ns | 6.58 ns | 8.46 ns | 8.45 ns |
| D230 | 17.7 ns | 17.7 ns | 16.2 ns | 12.9 ns | 14.9 ns |
| D307 | 23.4 ns | 23.4 ns | 18.8 ns | 23.3 ns | 25.2 ns |
| D462 | 37.4 ns | 41.2 ns | 43 ns | 37.2 ns | 40.5 ns |
| D616 | 74.9 ns | 46.2 ns | 33 ns | 45.3 ns | 46.5 ns |
| D924 | 76.5 ns | 84.8 ns | 87.1 ns | 84.7 ns | 75.3 ns |
| D1232 | 106 ns | 93.5 ns | 106 ns | 95.6 ns | 95.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.7 124.4,140.1 160.5,135.4 196.7,125.7 232.9,113.7 269.1,97.6 305.3,91.6 341.5,81.3 377.6,66.3 413.8,65.8 450.0,58.7 450.0,61.0 413.8,66.2 377.6,76.6 341.5,79.6 305.3,90.0 269.1,101.4 232.9,113.7 196.7,125.6 160.5,142.5 124.4,142.1 88.2,147.1 52.0,163.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.7 124.4,140.1 160.5,135.4 196.7,125.7 232.9,113.7 269.1,97.6 305.3,91.6 341.5,81.3 377.6,66.3 413.8,65.8 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,147.1 124.4,140.1 160.5,133.1 196.7,125.8 232.9,115.7 269.1,97.6 305.3,91.5 341.5,79.3 377.6,76.8 413.8,63.6 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,142.2 160.5,133.1 196.7,125.8 232.9,119.1 269.1,99.6 305.3,96.3 341.5,78.3 377.6,84.1 413.8,63.0 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,158.8 124.4,142.2 160.5,135.4 196.7,122.8 232.9,113.6 269.1,104.5 305.3,91.6 341.5,81.5 377.6,77.2 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.1 88.2,147.1 124.4,142.1 160.5,142.5 196.7,125.6 232.9,113.7 269.1,101.4 305.3,90.0 341.5,79.6 377.6,76.6 413.8,66.2 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
