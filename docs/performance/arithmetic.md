# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.98 ns | 0.936 ns | 1.05 ns | 0.945 ns | 1.05 ns |
| D38 | 1.61 ns | 1.14 ns | 0.982 ns | 1.82 ns | 1.61 ns |
| D57 | 2.89 ns | 1.27 ns | 2.49 ns | 2.49 ns | 1.26 ns |
| D76 | 1.49 ns | 1.82 ns | 3.08 ns | 1.79 ns | 4.07 ns |
| D115 | 5 ns | 2.84 ns | 3.34 ns | 4.97 ns | 4.99 ns |
| D153 | 3.92 ns | 5.91 ns | 3.78 ns | 6.62 ns | 3.37 ns |
| D230 | 15.4 ns | 11.9 ns | 15.3 ns | 11.9 ns | 13.9 ns |
| D307 | 19.6 ns | 18.6 ns | 12.2 ns | 18.5 ns | 19.6 ns |
| D462 | 28.9 ns | 32.6 ns | 29.3 ns | 28.9 ns | 27.3 ns |
| D616 | 50.9 ns | 34.1 ns | 45.4 ns | 44.9 ns | 58.4 ns |
| D924 | 84.8 ns | 84.9 ns | 53.7 ns | 85.5 ns | 74.5 ns |
| D1232 | 106 ns | 85.7 ns | 95.2 ns | 61 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.4 88.2,149.6 124.4,137.0 160.5,151.3 196.7,125.0 232.9,130.3 269.1,100.7 305.3,95.4 341.5,86.9 377.6,74.7 413.8,63.6 450.0,58.7 450.0,58.8 413.8,66.4 377.6,71.7 341.5,88.2 305.3,95.4 269.1,102.9 232.9,133.6 196.7,125.1 160.5,129.5 124.4,154.9 88.2,149.6 52.0,158.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.4 88.2,149.6 124.4,137.0 160.5,151.3 196.7,125.0 232.9,130.3 269.1,100.7 305.3,95.4 341.5,86.9 377.6,74.7 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,157.1 124.4,154.7 160.5,147.0 196.7,137.3 232.9,121.4 269.1,106.1 305.3,96.5 341.5,84.3 377.6,83.3 413.8,63.6 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,160.4 124.4,140.2 160.5,135.6 196.7,133.8 232.9,131.1 269.1,100.8 305.3,105.7 341.5,86.6 377.6,77.1 413.8,73.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.2 88.2,147.0 124.4,140.2 160.5,147.3 196.7,125.2 232.9,119.0 269.1,106.2 305.3,96.6 341.5,86.9 377.6,77.4 413.8,63.4 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.9 88.2,149.6 124.4,154.9 160.5,129.5 196.7,125.1 232.9,133.6 269.1,102.9 305.3,95.4 341.5,88.2 377.6,71.7 413.8,66.4 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.89 ns | 5.9 ns | 6.19 ns | 8.31 ns | 8.93 ns |
| D38 | 9.79 ns | 9.78 ns | 10.3 ns | 67.5 ns | 59.4 ns |
| D57 | 20.7 ns | 19.4 ns | 74.8 ns | 122 ns | 74.2 ns |
| D76 | 15.5 ns | 38.1 ns | 75.4 ns | 85.2 ns | 174 ns |
| D115 | 46.2 ns | 55.3 ns | 83.5 ns | 202 ns | 253 ns |
| D153 | 37.3 ns | 110 ns | 104 ns | 264 ns | 201 ns |
| D230 | 98.8 ns | 129 ns | 252 ns | 340 ns | 570 ns |
| D307 | 134 ns | 223 ns | 223 ns | 584 ns | 973 ns |
| D462 | 194 ns | 465 ns | 700 ns | 1.07 µs | 1.17 µs |
| D616 | 201 ns | 433 ns | 1.03 µs | 1.82 µs | 1.89 µs |
| D924 | 386 ns | 1.23 µs | 1.29 µs | 2.84 µs | 4.04 µs |
| D1232 | 551 ns | 1.56 µs | 3.46 µs | 2.85 µs | 7.77 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.6 88.2,160.5 124.4,144.2 160.5,150.5 196.7,126.8 232.9,131.4 269.1,110.3 305.3,103.7 341.5,95.7 377.6,94.9 413.8,80.7 450.0,72.9 450.0,15.5 413.8,29.7 377.6,46.1 341.5,56.7 305.3,60.6 269.1,72.2 232.9,94.8 196.7,89.9 160.5,98.0 124.4,116.5 88.2,121.3 52.0,162.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.6 88.2,160.5 124.4,144.2 160.5,150.5 196.7,126.8 232.9,131.4 269.1,110.3 305.3,103.7 341.5,95.7 377.6,94.9 413.8,80.7 450.0,72.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,160.5 124.4,145.6 160.5,130.9 196.7,122.8 232.9,107.9 269.1,104.4 305.3,92.5 341.5,76.6 377.6,78.2 413.8,55.5 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,159.3 124.4,116.3 160.5,116.1 196.7,113.9 232.9,109.1 269.1,89.9 305.3,92.6 341.5,67.8 377.6,59.3 413.8,54.5 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.0 88.2,118.6 124.4,105.7 160.5,113.5 196.7,94.7 232.9,88.9 269.1,83.4 305.3,71.7 341.5,58.6 377.6,47.0 413.8,37.3 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.5 88.2,121.3 124.4,116.5 160.5,98.0 196.7,89.9 232.9,94.8 269.1,72.2 305.3,60.6 341.5,56.7 377.6,46.1 413.8,29.7 450.0,15.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.653 ns | 3 ns | 3.2 ns | 4.92 ns | 4.97 ns |
| D38 | 3.5 ns | 12.7 ns | 17.8 ns | 30.5 ns | 27.7 ns |
| D57 | 4.16 ns | 12.8 ns | 34.4 ns | 76.2 ns | 46.3 ns |
| D76 | 3 ns | 25.1 ns | 43.2 ns | 55.9 ns | 128 ns |
| D115 | 13.6 ns | 31 ns | 67.6 ns | 213 ns | 255 ns |
| D153 | 11.5 ns | 50.4 ns | 78 ns | 259 ns | 205 ns |
| D230 | 28.5 ns | 101 ns | 368 ns | 440 ns | 1.02 µs |
| D307 | 54.3 ns | 167 ns | 292 ns | 1.03 µs | 1.47 µs |
| D462 | 91.4 ns | 449 ns | 1.27 µs | 1.77 µs | 2.05 µs |
| D616 | 69.1 ns | 433 ns | 1.77 µs | 2.73 µs | 3.28 µs |
| D924 | 156 ns | 1.58 µs | 1.67 µs | 5.45 µs | 7.7 µs |
| D1232 | 196 ns | 1.86 µs | 4.72 µs | 5.86 µs | 14.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.8 88.2,158.5 124.4,156.0 160.5,160.8 196.7,138.9 232.9,141.3 269.1,128.2 305.3,118.8 341.5,111.3 377.6,115.3 413.8,103.6 450.0,100.2 450.0,38.3 413.8,47.1 377.6,59.5 341.5,66.3 305.3,71.1 269.1,76.4 232.9,99.6 196.7,96.5 160.5,106.5 124.4,121.1 88.2,128.6 52.0,153.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.8 88.2,158.5 124.4,156.0 160.5,160.8 196.7,138.9 232.9,141.3 269.1,128.2 305.3,118.8 341.5,111.3 377.6,115.3 413.8,103.6 450.0,100.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.8 88.2,139.9 124.4,139.8 160.5,130.0 196.7,126.9 232.9,119.9 269.1,109.8 305.3,102.6 341.5,88.3 377.6,88.8 413.8,70.0 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,135.0 124.4,125.4 160.5,122.2 196.7,115.7 232.9,113.6 269.1,91.1 305.3,94.5 341.5,73.2 377.6,68.4 413.8,69.3 450.0,54.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,127.2 124.4,113.9 160.5,118.4 196.7,99.0 232.9,96.2 269.1,88.5 305.3,76.2 341.5,68.4 377.6,62.1 413.8,52.1 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,128.6 124.4,121.1 160.5,106.5 196.7,96.5 232.9,99.6 269.1,76.4 305.3,71.1 341.5,66.3 377.6,59.5 413.8,47.1 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.542 ns | 0.622 ns | 0.704 ns | 0.623 ns | 0.703 ns |
| D38 | 1.32 ns | 1.08 ns | 0.932 ns | 1.45 ns | 1.32 ns |
| D57 | 1.57 ns | 1.05 ns | 1.74 ns | 1.74 ns | 1.05 ns |
| D76 | 1.12 ns | 1.5 ns | 2.09 ns | 1.45 ns | 3.07 ns |
| D115 | 3.17 ns | 2.47 ns | 2.85 ns | 3.55 ns | 3.55 ns |
| D153 | 2.78 ns | 3.83 ns | 2.8 ns | 4.6 ns | 2.38 ns |
| D230 | 6.65 ns | 5.16 ns | 7.24 ns | 5.62 ns | 7.16 ns |
| D307 | 12.3 ns | 11 ns | 5.63 ns | 11 ns | 12.4 ns |
| D462 | 15 ns | 16.7 ns | 15.3 ns | 15.2 ns | 13.9 ns |
| D616 | 21.2 ns | 17.2 ns | 20.2 ns | 20.2 ns | 18.2 ns |
| D924 | 63.2 ns | 84.7 ns | 46.7 ns | 84.8 ns | 76.3 ns |
| D1232 | 54.5 ns | 50.6 ns | 61.5 ns | 36.5 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.1 88.2,135.2 124.4,130.3 160.5,140.0 196.7,110.0 232.9,113.7 269.1,88.5 305.3,70.7 341.5,64.9 377.6,55.0 413.8,23.3 450.0,27.6 450.0,20.4 413.8,17.8 377.6,59.3 341.5,67.1 305.3,70.5 269.1,86.3 232.9,118.2 196.7,106.6 160.5,110.9 124.4,142.0 88.2,135.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.1 88.2,135.2 124.4,130.3 160.5,140.0 196.7,110.0 232.9,113.7 269.1,88.5 305.3,70.7 341.5,64.9 377.6,55.0 413.8,23.3 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,141.0 124.4,142.0 160.5,131.7 196.7,117.2 232.9,104.5 269.1,95.8 305.3,74.0 341.5,61.9 377.6,61.0 413.8,14.8 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,145.4 124.4,127.3 160.5,121.9 196.7,113.0 232.9,113.5 269.1,86.0 305.3,93.3 341.5,64.3 377.6,56.4 413.8,32.1 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,132.7 124.4,127.3 160.5,132.6 196.7,106.6 232.9,99.1 269.1,93.4 305.3,74.0 341.5,64.6 377.6,56.3 413.8,14.8 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,142.0 160.5,110.9 196.7,106.6 232.9,118.2 269.1,86.3 305.3,70.5 341.5,67.1 377.6,59.3 413.8,17.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.68 ns | 1.87 ns | 2.11 ns | 2.18 ns | 2.46 ns |
| D38 | 3.25 ns | 2.48 ns | 2.11 ns | 3.54 ns | 3.16 ns |
| D57 | 6.33 ns | 3.69 ns | 8.08 ns | 8.08 ns | 3.45 ns |
| D76 | 4.13 ns | 5.38 ns | 8.48 ns | 4.85 ns | 11.1 ns |
| D115 | 14.4 ns | 7.81 ns | 8.77 ns | 14.4 ns | 14.4 ns |
| D153 | 10.9 ns | 16 ns | 10.6 ns | 20 ns | 8.62 ns |
| D230 | 36.5 ns | 28.2 ns | 36 ns | 28.1 ns | 32.1 ns |
| D307 | 48.3 ns | 40.3 ns | 23.5 ns | 42.7 ns | 47.9 ns |
| D462 | 74.2 ns | 85.4 ns | 81.6 ns | 72.5 ns | 66.3 ns |
| D616 | 77.9 ns | 53.8 ns | 78.7 ns | 77.7 ns | 88.8 ns |
| D924 | 109 ns | 108 ns | 57.7 ns | 104 ns | 82.4 ns |
| D1232 | 149 ns | 110 ns | 119 ns | 59.9 ns | 122 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.0 88.2,175.9 124.4,156.6 160.5,169.0 196.7,132.7 232.9,140.8 269.1,105.9 305.3,97.7 341.5,85.3 377.6,83.9 413.8,74.3 450.0,65.1 450.0,70.8 413.8,82.3 377.6,80.1 341.5,88.6 305.3,98.0 269.1,109.6 232.9,147.6 196.7,132.8 160.5,140.3 124.4,174.1 88.2,176.7 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.0 88.2,175.9 124.4,156.6 160.5,169.0 196.7,132.7 232.9,140.8 269.1,105.9 305.3,97.7 341.5,85.3 377.6,83.9 413.8,74.3 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,183.8 124.4,172.2 160.5,161.3 196.7,150.5 232.9,129.7 269.1,113.3 305.3,103.0 341.5,81.3 377.6,94.6 413.8,74.4 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,188.4 124.4,149.5 160.5,148.1 196.7,147.1 232.9,141.5 269.1,106.2 305.3,118.6 341.5,82.6 377.6,83.6 413.8,92.6 450.0,71.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,173.4 124.4,149.5 160.5,164.3 196.7,132.7 232.9,123.2 269.1,113.4 305.3,101.3 341.5,86.0 377.6,84.0 413.8,75.7 450.0,91.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,176.7 124.4,174.1 160.5,140.3 196.7,132.8 232.9,147.6 269.1,109.6 305.3,98.0 341.5,88.6 377.6,80.1 413.8,82.3 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.979 ns | 0.935 ns | 1.06 ns | 0.939 ns | 1.05 ns |
| D38 | 1.61 ns | 1.05 ns | 0.9 ns | 1.81 ns | 1.6 ns |
| D57 | 2.94 ns | 1.3 ns | 2.5 ns | 2.51 ns | 1.3 ns |
| D76 | 1.64 ns | 2.14 ns | 3.09 ns | 2.1 ns | 4.04 ns |
| D115 | 5.57 ns | 3.6 ns | 4.19 ns | 5.54 ns | 5.56 ns |
| D153 | 5.22 ns | 7.55 ns | 5.14 ns | 8.45 ns | 4.39 ns |
| D230 | 17.7 ns | 13.7 ns | 17.6 ns | 13.7 ns | 16.2 ns |
| D307 | 25.1 ns | 23.4 ns | 13.5 ns | 23.4 ns | 25.2 ns |
| D462 | 37.2 ns | 40.4 ns | 38.8 ns | 37.2 ns | 34.2 ns |
| D616 | 53.9 ns | 37.5 ns | 46.1 ns | 46.2 ns | 60.1 ns |
| D924 | 85.1 ns | 84.8 ns | 52.8 ns | 85 ns | 75 ns |
| D1232 | 106 ns | 94.3 ns | 95.7 ns | 68.2 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.5 88.2,149.6 124.4,136.6 160.5,149.3 196.7,122.7 232.9,124.1 269.1,97.6 305.3,90.0 341.5,81.5 377.6,73.4 413.8,63.5 450.0,58.7 450.0,58.7 413.8,66.2 377.6,71.0 341.5,83.3 305.3,90.0 269.1,99.6 232.9,127.9 196.7,122.7 160.5,129.7 124.4,154.4 88.2,149.8 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.5 88.2,149.6 124.4,136.6 160.5,149.3 196.7,122.7 232.9,124.1 269.1,97.6 305.3,90.0 341.5,81.5 377.6,73.4 413.8,63.5 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,158.9 124.4,154.3 160.5,143.4 196.7,132.2 232.9,116.1 269.1,103.2 305.3,91.5 341.5,79.7 377.6,81.3 413.8,63.6 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,162.3 124.4,140.1 160.5,135.5 196.7,128.9 232.9,124.5 269.1,97.7 305.3,103.5 341.5,80.5 377.6,76.8 413.8,73.9 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.1 124.4,140.0 160.5,143.9 196.7,122.8 232.9,113.7 269.1,103.1 305.3,91.5 341.5,81.5 377.6,76.8 413.8,63.5 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.8 124.4,154.4 160.5,129.7 196.7,122.7 232.9,127.9 269.1,99.6 305.3,90.0 341.5,83.3 377.6,71.0 413.8,66.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
