# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.934 ns | 0.935 ns | 0.936 ns | 0.942 ns | 1.05 ns |
| D38 | 1.62 ns | 1.61 ns | 1.82 ns | 1.62 ns | 1.62 ns |
| D57 | 2.25 ns | 2.5 ns | 2.25 ns | 2.25 ns | 2.5 ns |
| D76 | 3.1 ns | 2.23 ns | 3.09 ns | 2.19 ns | 3.09 ns |
| D115 | 4.41 ns | 2.52 ns | 4.4 ns | 4.99 ns | 5 ns |
| D153 | 5.9 ns | 5.14 ns | 6.65 ns | 5.91 ns | 5.91 ns |
| D230 | 15.3 ns | 13.9 ns | 13.8 ns | 10.2 ns | 13.9 ns |
| D307 | 18.5 ns | 18.5 ns | 15.2 ns | 18.6 ns | 18.6 ns |
| D462 | 33.3 ns | 40.4 ns | 49.7 ns | 29.1 ns | 33.4 ns |
| D616 | 34.1 ns | 74.8 ns | 49.9 ns | 45.4 ns | 50.8 ns |
| D924 | 85 ns | 83.2 ns | 75.1 ns | 79.1 ns | 58.9 ns |
| D1232 | 95 ns | 95 ns | 106 ns | 79.2 ns | 60.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.5 124.4,142.4 160.5,135.5 196.7,127.8 232.9,121.5 269.1,100.7 305.3,96.6 341.5,83.9 377.6,83.4 413.8,63.5 450.0,61.1 450.0,71.0 413.8,71.5 377.6,74.7 341.5,83.8 305.3,96.5 269.1,102.9 232.9,121.4 196.7,125.1 160.5,135.5 124.4,140.1 88.2,149.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.5 124.4,142.4 160.5,135.5 196.7,127.8 232.9,121.5 269.1,100.7 305.3,96.6 341.5,83.9 377.6,83.4 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,142.6 196.7,139.9 232.9,124.5 269.1,102.9 305.3,96.6 341.5,79.7 377.6,66.3 413.8,64.0 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,142.4 160.5,135.5 196.7,127.8 232.9,118.9 269.1,103.0 305.3,101.0 341.5,75.2 377.6,75.1 413.8,66.2 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,149.5 124.4,142.3 160.5,143.0 196.7,125.1 232.9,121.4 269.1,109.5 305.3,96.5 341.5,86.8 377.6,77.2 413.8,65.1 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,140.1 160.5,135.5 196.7,125.1 232.9,121.4 269.1,102.9 305.3,96.5 341.5,83.8 377.6,74.7 413.8,71.5 450.0,71.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.09 ns | 5.42 ns | 5.61 ns | 7.71 ns | 8.93 ns |
| D38 | 9.88 ns | 11.6 ns | 14.3 ns | 58.5 ns | 59.7 ns |
| D57 | 24 ns | 34.6 ns | 68.3 ns | 111 ns | 120 ns |
| D76 | 26.5 ns | 45.2 ns | 75.9 ns | 94.9 ns | 141 ns |
| D115 | 43.2 ns | 48.4 ns | 104 ns | 201 ns | 253 ns |
| D153 | 54.8 ns | 94.9 ns | 156 ns | 245 ns | 325 ns |
| D230 | 93 ns | 154 ns | 231 ns | 272 ns | 568 ns |
| D307 | 118 ns | 229 ns | 312 ns | 591 ns | 885 ns |
| D462 | 220 ns | 476 ns | 699 ns | 1.06 µs | 1.47 µs |
| D616 | 156 ns | 668 ns | 1.03 µs | 1.81 µs | 2.39 µs |
| D924 | 388 ns | 1.1 µs | 2.06 µs | 2.3 µs | 3.11 µs |
| D1232 | 508 ns | 1.75 µs | 3.81 µs | 3.98 µs | 4.02 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.8 88.2,160.3 124.4,141.0 160.5,138.8 196.7,128.2 232.9,123.1 269.1,111.6 305.3,106.4 341.5,92.9 377.6,100.3 413.8,80.6 450.0,74.7 450.0,29.8 413.8,35.4 377.6,41.0 341.5,51.6 305.3,62.6 269.1,72.3 232.9,84.4 196.7,89.8 160.5,102.5 124.4,106.0 88.2,121.2 52.0,162.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.8 88.2,160.3 124.4,141.0 160.5,138.8 196.7,128.2 232.9,123.1 269.1,111.6 305.3,106.4 341.5,92.9 377.6,100.3 413.8,80.6 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.3 88.2,156.7 124.4,133.1 160.5,127.2 196.7,125.7 232.9,111.1 269.1,100.6 305.3,92.0 341.5,76.1 377.6,68.8 413.8,57.9 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.6 88.2,152.2 124.4,118.3 160.5,116.0 196.7,109.1 232.9,100.3 269.1,91.8 305.3,85.3 341.5,67.8 377.6,59.4 413.8,44.3 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.7 88.2,121.6 124.4,107.7 160.5,111.1 196.7,94.8 232.9,90.6 269.1,88.3 305.3,71.4 341.5,58.8 377.6,47.1 413.8,41.9 450.0,30.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.5 88.2,121.2 124.4,106.0 160.5,102.5 196.7,89.8 232.9,84.4 269.1,72.3 305.3,62.6 341.5,51.6 377.6,41.0 413.8,35.4 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.882 ns | 2.87 ns | 3.13 ns | 4.95 ns | 4.97 ns |
| D38 | 3.5 ns | 14 ns | 29.5 ns | 27.4 ns | 27.6 ns |
| D57 | 6.24 ns | 21.9 ns | 33.5 ns | 72.5 ns | 77.1 ns |
| D76 | 8.1 ns | 29.6 ns | 42.2 ns | 66.6 ns | 102 ns |
| D115 | 13.3 ns | 26 ns | 85.5 ns | 215 ns | 252 ns |
| D153 | 18.4 ns | 45.2 ns | 120 ns | 236 ns | 356 ns |
| D230 | 28.4 ns | 116 ns | 339 ns | 348 ns | 997 ns |
| D307 | 44 ns | 166 ns | 394 ns | 1.05 µs | 1.39 µs |
| D462 | 102 ns | 460 ns | 1.26 µs | 1.75 µs | 2.62 µs |
| D616 | 67.1 ns | 724 ns | 1.77 µs | 2.74 µs | 4.18 µs |
| D924 | 163 ns | 1.47 µs | 3.01 µs | 4.29 µs | 5.41 µs |
| D1232 | 192 ns | 2.25 µs | 5.13 µs | 7.65 µs | 7.14 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,172.2 88.2,148.2 124.4,138.2 160.5,133.7 196.7,125.0 232.9,119.4 269.1,111.8 305.3,104.3 341.5,89.6 377.6,96.9 413.8,81.5 450.0,78.7 450.0,15.9 413.8,20.7 377.6,25.1 341.5,33.3 305.3,44.3 269.1,50.0 232.9,67.9 196.7,73.9 160.5,89.6 124.4,94.5 88.2,112.4 52.0,142.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,172.2 88.2,148.2 124.4,138.2 160.5,133.7 196.7,125.0 232.9,119.4 269.1,111.8 305.3,104.3 341.5,89.6 377.6,96.9 413.8,81.5 450.0,78.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,151.7 88.2,124.1 124.4,116.4 160.5,111.1 196.7,113.4 232.9,103.8 269.1,87.3 305.3,81.1 341.5,63.5 377.6,55.6 413.8,43.3 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.2 88.2,111.2 124.4,109.0 160.5,105.0 196.7,92.7 232.9,86.8 269.1,68.8 305.3,66.2 341.5,46.0 377.6,40.1 413.8,30.9 450.0,21.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,142.2 88.2,112.5 124.4,95.6 160.5,97.1 196.7,76.7 232.9,75.1 269.1,68.4 305.3,49.2 341.5,40.3 377.6,32.5 413.8,24.7 450.0,14.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,142.1 88.2,112.4 124.4,94.5 160.5,89.6 196.7,73.9 232.9,67.9 269.1,50.0 305.3,44.3 341.5,33.3 377.6,25.1 413.8,20.7 450.0,15.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.623 ns | 0.623 ns | 0.622 ns | 0.704 ns |
| D38 | 1.33 ns | 1.32 ns | 1.45 ns | 1.33 ns | 1.32 ns |
| D57 | 1.87 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.74 ns |
| D76 | 2.19 ns | 1.82 ns | 2.18 ns | 1.74 ns | 2.49 ns |
| D115 | 2.83 ns | 1.9 ns | 2.82 ns | 3.55 ns | 3.55 ns |
| D153 | 3.78 ns | 3.27 ns | 4.6 ns | 4.49 ns | 4.48 ns |
| D230 | 6.65 ns | 5.87 ns | 7.16 ns | 4.13 ns | 7.17 ns |
| D307 | 10.9 ns | 11.1 ns | 9.58 ns | 11.1 ns | 11.1 ns |
| D462 | 17.5 ns | 16.7 ns | 24.6 ns | 15.5 ns | 17 ns |
| D616 | 13 ns | 21.6 ns | 20.3 ns | 20.2 ns | 21.9 ns |
| D924 | 63.3 ns | 82 ns | 75.4 ns | 90.7 ns | 47.5 ns |
| D1232 | 47.6 ns | 61.5 ns | 69.7 ns | 52.6 ns | 36.3 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,120.7 196.7,113.2 232.9,104.8 269.1,88.5 305.3,74.2 341.5,60.4 377.6,69.2 413.8,23.3 450.0,31.5 450.0,39.4 413.8,31.6 377.6,54.0 341.5,61.2 305.3,73.7 269.1,86.3 232.9,99.9 196.7,106.7 160.5,117.0 124.4,127.3 88.2,135.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,120.7 196.7,113.2 232.9,104.8 269.1,88.5 305.3,74.2 341.5,60.4 377.6,69.2 413.8,23.3 450.0,31.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.0 88.2,135.2 124.4,127.3 160.5,126.0 196.7,124.8 232.9,109.0 269.1,92.1 305.3,73.7 341.5,61.9 377.6,54.3 413.8,15.7 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,132.6 124.4,125.2 160.5,120.7 196.7,113.3 232.9,99.1 269.1,86.3 305.3,77.9 341.5,50.6 377.6,56.2 413.8,18.2 450.0,20.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,127.4 196.7,106.7 232.9,99.9 269.1,102.3 305.3,73.7 341.5,64.0 377.6,56.3 413.8,12.8 450.0,28.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,117.0 196.7,106.7 232.9,99.9 269.1,86.3 305.3,73.7 341.5,61.2 377.6,54.0 413.8,31.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 1.87 ns | 2.18 ns | 2.46 ns |
| D38 | 3.25 ns | 3.26 ns | 3.55 ns | 3.17 ns | 3.16 ns |
| D57 | 7.17 ns | 8.09 ns | 7.17 ns | 7.16 ns | 8.08 ns |
| D76 | 8.7 ns | 6.85 ns | 8.71 ns | 6.19 ns | 8.42 ns |
| D115 | 12.7 ns | 7.28 ns | 12.8 ns | 14.4 ns | 14.4 ns |
| D153 | 16 ns | 15.8 ns | 20 ns | 16.1 ns | 17 ns |
| D230 | 39 ns | 32.3 ns | 31.8 ns | 17.2 ns | 32 ns |
| D307 | 41.6 ns | 40.7 ns | 37.2 ns | 42.6 ns | 42.5 ns |
| D462 | 89.9 ns | 88.6 ns | 92.3 ns | 72.9 ns | 83.5 ns |
| D616 | 54.1 ns | 104 ns | 81.2 ns | 78.5 ns | 93.2 ns |
| D924 | 111 ns | 101 ns | 101 ns | 117 ns | 61.3 ns |
| D1232 | 133 ns | 133 ns | 130 ns | 84.4 ns | 78.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,147.4 196.7,136.3 232.9,129.7 269.1,103.9 305.3,102.0 341.5,79.7 377.6,94.4 413.8,73.6 450.0,68.5 450.0,83.7 413.8,90.8 377.6,78.7 341.5,81.9 305.3,101.4 269.1,109.6 232.9,128.0 196.7,132.8 160.5,148.3 124.4,149.5 88.2,176.7 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,147.4 196.7,136.3 232.9,129.7 269.1,103.9 305.3,102.0 341.5,79.7 377.6,94.4 413.8,73.6 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.8 124.4,149.5 160.5,154.3 196.7,152.5 232.9,130.2 269.1,109.4 305.3,102.7 341.5,80.2 377.6,75.5 413.8,76.2 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,173.4 124.4,153.0 160.5,147.3 196.7,136.3 232.9,123.2 269.1,109.8 305.3,105.3 341.5,79.0 377.6,82.7 413.8,76.5 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.6 124.4,153.0 160.5,157.2 196.7,132.8 232.9,129.6 269.1,127.6 305.3,101.3 341.5,85.8 377.6,83.7 413.8,72.2 450.0,81.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,176.7 124.4,149.5 160.5,148.3 196.7,132.8 232.9,128.0 269.1,109.6 305.3,101.4 341.5,81.9 377.6,78.7 413.8,90.8 450.0,83.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.934 ns | 0.934 ns | 0.934 ns | 0.937 ns | 1.06 ns |
| D38 | 1.61 ns | 1.6 ns | 1.81 ns | 1.61 ns | 1.61 ns |
| D57 | 2.27 ns | 2.5 ns | 2.28 ns | 2.27 ns | 2.5 ns |
| D76 | 3.08 ns | 2.6 ns | 3.09 ns | 2.57 ns | 3.09 ns |
| D115 | 4.87 ns | 3.06 ns | 4.91 ns | 5.55 ns | 5.55 ns |
| D153 | 7.65 ns | 6.57 ns | 8.46 ns | 7.65 ns | 7.64 ns |
| D230 | 17.7 ns | 16.2 ns | 16.1 ns | 11.3 ns | 16.2 ns |
| D307 | 23.3 ns | 23.3 ns | 19.5 ns | 23.4 ns | 23.4 ns |
| D462 | 43.3 ns | 47.9 ns | 53.8 ns | 37.5 ns | 43.2 ns |
| D616 | 37.5 ns | 73.3 ns | 50.9 ns | 45.8 ns | 48.9 ns |
| D924 | 85 ns | 83.8 ns | 74.9 ns | 80.1 ns | 62.3 ns |
| D1232 | 95.1 ns | 95 ns | 106 ns | 89 ns | 60 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.7 124.4,142.2 160.5,135.5 196.7,125.6 232.9,115.8 269.1,97.6 305.3,91.6 341.5,78.2 377.6,81.3 413.8,63.5 450.0,61.1 450.0,71.1 413.8,70.3 377.6,75.6 341.5,78.2 305.3,91.6 269.1,99.6 232.9,115.8 196.7,122.8 160.5,135.5 124.4,140.1 88.2,149.7 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.7 124.4,142.2 160.5,135.5 196.7,125.6 232.9,115.8 269.1,97.6 305.3,91.6 341.5,78.2 377.6,81.3 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.8 124.4,140.1 160.5,139.2 196.7,135.7 232.9,119.1 269.1,99.5 305.3,91.6 341.5,76.0 377.6,66.8 413.8,63.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.1 124.4,142.1 160.5,135.5 196.7,125.5 232.9,113.6 269.1,99.6 305.3,95.5 341.5,73.4 377.6,74.7 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,142.2 160.5,139.5 196.7,122.8 232.9,115.8 269.1,107.4 305.3,91.6 341.5,81.3 377.6,77.0 413.8,64.8 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,140.1 160.5,135.5 196.7,122.8 232.9,115.8 269.1,99.6 305.3,91.6 341.5,78.2 377.6,75.6 413.8,70.3 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
