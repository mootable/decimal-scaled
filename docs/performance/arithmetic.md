# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.934 ns | 0.932 ns | 1.15 ns | 0.935 ns | 0.936 ns |
| D38 | 1.62 ns | 1.82 ns | 1.83 ns | 1.82 ns | 1.83 ns |
| D57 | 2.49 ns | 2.25 ns | 2.5 ns | 2.25 ns | 2.49 ns |
| D76 | 3.48 ns | 3.47 ns | 3.48 ns | 3.5 ns | 3.08 ns |
| D115 | 3.87 ns | 4.42 ns | 4.41 ns | 4.41 ns | 4.99 ns |
| D153 | 3.15 ns | 5.94 ns | 5.94 ns | 5.94 ns | 5.94 ns |
| D230 | 15.3 ns | 15.4 ns | 15.4 ns | 13.8 ns | 13.8 ns |
| D307 | 19.6 ns | 19.7 ns | 18.5 ns | 18.6 ns | 18.5 ns |
| D462 | 29.6 ns | 32.6 ns | 33.2 ns | 32.5 ns | 32.5 ns |
| D616 | 49.9 ns | 49.2 ns | 60.7 ns | 60.7 ns | 72.6 ns |
| D924 | 76.4 ns | 85 ns | 74.7 ns | 84.9 ns | 79.6 ns |
| D1232 | 106 ns | 85.5 ns | 63.6 ns | 95.1 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.5 124.4,140.2 160.5,132.9 196.7,130.6 232.9,135.1 269.1,100.7 305.3,95.4 341.5,86.4 377.6,75.1 413.8,65.8 450.0,58.6 450.0,58.6 413.8,64.9 377.6,66.9 341.5,84.4 305.3,96.6 269.1,102.9 232.9,121.3 196.7,125.1 160.5,135.6 124.4,140.2 88.2,146.9 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.5 124.4,140.2 160.5,132.9 196.7,130.6 232.9,135.1 269.1,100.7 305.3,95.4 341.5,86.4 377.6,75.1 413.8,65.8 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,147.0 124.4,142.4 160.5,133.0 196.7,127.7 232.9,121.3 269.1,100.7 305.3,95.3 341.5,84.4 377.6,75.4 413.8,63.5 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,146.8 124.4,140.1 160.5,132.9 196.7,127.8 232.9,121.3 269.1,100.6 305.3,96.6 341.5,83.9 377.6,70.8 413.8,66.3 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,132.8 196.7,127.8 232.9,121.3 269.1,102.9 305.3,96.6 341.5,84.4 377.6,70.8 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,146.9 124.4,140.2 160.5,135.6 196.7,125.1 232.9,121.3 269.1,102.9 305.3,96.6 341.5,84.4 377.6,66.9 413.8,64.9 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.97 ns | 5.68 ns | 6.43 ns | 8.53 ns | 8.6 ns |
| D38 | 9.54 ns | 12.4 ns | 14.1 ns | 809 ns | 1.07 µs |
| D57 | 35.3 ns | 49.2 ns | 74.9 ns | 100 ns | 107 ns |
| D76 | 41 ns | 65.6 ns | 85.4 ns | 106 ns | 125 ns |
| D115 | 45.5 ns | 82.5 ns | 104 ns | 169 ns | 244 ns |
| D153 | 33.9 ns | 108 ns | 143 ns | 222 ns | 299 ns |
| D230 | 110 ns | 168 ns | 249 ns | 364 ns | 535 ns |
| D307 | 146 ns | 240 ns | 359 ns | 560 ns | 866 ns |
| D462 | 210 ns | 446 ns | 700 ns | 1.1 µs | 1.45 µs |
| D616 | 271 ns | 601 ns | 1.05 µs | 1.95 µs | 2.35 µs |
| D924 | 352 ns | 1.17 µs | 2.02 µs | 2.82 µs | 4.45 µs |
| D1232 | 554 ns | 1.51 µs | 2.22 µs | 4.13 µs | 7.84 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.2 88.2,161.0 124.4,132.6 160.5,129.3 196.7,127.1 232.9,133.5 269.1,107.9 305.3,101.7 341.5,93.8 377.6,88.3 413.8,82.7 450.0,72.8 450.0,15.3 413.8,27.6 377.6,41.4 341.5,51.9 305.3,63.1 269.1,73.6 232.9,86.3 196.7,90.6 160.5,105.1 124.4,108.6 88.2,58.6 52.0,163.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,161.0 124.4,132.6 160.5,129.3 196.7,127.1 232.9,133.5 269.1,107.9 305.3,101.7 341.5,93.8 377.6,88.3 413.8,82.7 450.0,72.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,172.3 88.2,155.4 124.4,125.4 160.5,119.1 196.7,114.2 232.9,108.2 269.1,98.7 305.3,91.0 341.5,77.5 377.6,71.0 413.8,56.5 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.6 88.2,152.6 124.4,116.3 160.5,113.4 196.7,109.1 232.9,102.2 269.1,90.2 305.3,82.3 341.5,67.8 377.6,58.9 413.8,44.8 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.4 88.2,64.6 124.4,109.9 160.5,108.8 196.7,98.6 232.9,92.7 269.1,82.0 305.3,72.6 341.5,57.9 377.6,45.5 413.8,37.5 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.3 88.2,58.6 124.4,108.6 160.5,105.1 196.7,90.6 232.9,86.3 269.1,73.6 305.3,63.1 341.5,51.9 377.6,41.4 413.8,27.6 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.99 ns | 3.02 ns | 4.92 ns | 5.09 ns |
| D38 | 3.5 ns | 13.7 ns | 30 ns | 33.3 ns | 30.9 ns |
| D57 | 4.22 ns | 20.8 ns | 35.9 ns | 71.5 ns | 77.1 ns |
| D76 | 5.64 ns | 38.5 ns | 43.6 ns | 85.4 ns | 103 ns |
| D115 | 10.5 ns | 47.6 ns | 94.3 ns | 194 ns | 252 ns |
| D153 | 8.44 ns | 52.7 ns | 112 ns | 234 ns | 353 ns |
| D230 | 28 ns | 125 ns | 371 ns | 517 ns | 980 ns |
| D307 | 55 ns | 188 ns | 459 ns | 1.05 µs | 1.41 µs |
| D462 | 83.4 ns | 449 ns | 1.31 µs | 1.87 µs | 2.61 µs |
| D616 | 92.1 ns | 643 ns | 1.85 µs | 2.9 µs | 4.18 µs |
| D924 | 146 ns | 1.63 µs | 2.95 µs | 5.41 µs | 7.61 µs |
| D1232 | 199 ns | 1.87 µs | 2.57 µs | 8.05 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,151.6 196.7,142.6 232.9,145.8 269.1,128.4 305.3,118.7 341.5,112.6 377.6,111.2 413.8,104.5 450.0,100.1 450.0,38.3 413.8,47.3 377.6,56.0 341.5,62.8 305.3,71.7 269.1,77.0 232.9,91.7 196.7,96.6 160.5,109.6 124.4,113.8 88.2,127.0 52.0,153.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,151.6 196.7,142.6 232.9,145.8 269.1,128.4 305.3,118.7 341.5,112.6 377.6,111.2 413.8,104.5 450.0,100.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.8 88.2,138.8 124.4,132.7 160.5,123.8 196.7,120.8 232.9,119.3 269.1,106.8 305.3,100.9 341.5,88.3 377.6,83.1 413.8,69.6 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.7 88.2,127.4 124.4,124.8 160.5,122.0 196.7,110.9 232.9,108.4 269.1,91.0 305.3,87.9 341.5,72.7 377.6,67.8 413.8,61.0 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,125.9 124.4,114.9 160.5,112.3 196.7,100.4 232.9,97.7 269.1,86.2 305.3,76.0 341.5,67.6 377.6,61.2 413.8,52.2 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.1 88.2,127.0 124.4,113.8 160.5,109.6 196.7,96.6 232.9,91.7 269.1,77.0 305.3,71.7 341.5,62.8 377.6,56.0 413.8,47.3 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.61 ns | 0.346 ns | 0.622 ns | 0.622 ns |
| D38 | 1.33 ns | 1.45 ns | 1.45 ns | 1.45 ns | 1.45 ns |
| D57 | 1.74 ns | 1.87 ns | 1.74 ns | 1.87 ns | 1.74 ns |
| D76 | 2.16 ns | 2.16 ns | 2.16 ns | 2.16 ns | 2.49 ns |
| D115 | 2.46 ns | 2.86 ns | 2.87 ns | 3.25 ns | 3.55 ns |
| D153 | 1.88 ns | 3.82 ns | 4.29 ns | 4.3 ns | 4.29 ns |
| D230 | 6.66 ns | 6.65 ns | 7.24 ns | 7.17 ns | 7.16 ns |
| D307 | 12.3 ns | 12.5 ns | 11.1 ns | 11.1 ns | 11.1 ns |
| D462 | 15.1 ns | 16.7 ns | 17 ns | 16.7 ns | 16.6 ns |
| D616 | 19 ns | 25.1 ns | 21.9 ns | 21.7 ns | 21.8 ns |
| D924 | 54.9 ns | 84.9 ns | 77.1 ns | 84.8 ns | 76.8 ns |
| D1232 | 54.5 ns | 51.9 ns | 41.2 ns | 61.4 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,121.1 196.7,117.3 232.9,125.0 269.1,88.4 305.3,70.8 341.5,64.8 377.6,58.2 413.8,27.4 450.0,27.6 450.0,20.4 413.8,17.6 377.6,54.1 341.5,61.9 305.3,73.7 269.1,86.3 232.9,101.1 196.7,106.6 160.5,117.0 124.4,127.3 88.2,132.6 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,121.1 196.7,117.3 232.9,125.0 269.1,88.4 305.3,70.8 341.5,64.8 377.6,58.2 413.8,27.4 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.6 88.2,132.6 124.4,125.2 160.5,121.0 196.7,112.9 232.9,104.5 269.1,88.5 305.3,70.2 341.5,61.9 377.6,50.0 413.8,14.7 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,132.6 124.4,127.3 160.5,121.1 196.7,112.9 232.9,101.1 269.1,86.0 305.3,73.7 341.5,61.3 377.6,54.0 413.8,17.5 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,125.2 160.5,121.1 196.7,109.2 232.9,101.1 269.1,86.3 305.3,73.7 341.5,61.9 377.6,54.2 413.8,14.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,127.3 160.5,117.0 196.7,106.6 232.9,101.1 269.1,86.3 305.3,73.7 341.5,61.9 377.6,54.1 413.8,17.6 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.81 ns | 1.73 ns | 2.18 ns | 2.18 ns |
| D38 | 7.18 ns | 8.1 ns | 14.8 ns | 19.3 ns | 16.6 ns |
| D57 | 8.09 ns | 7.17 ns | 8.09 ns | 7.17 ns | 8.09 ns |
| D76 | 9.83 ns | 9.84 ns | 9.84 ns | 9.77 ns | 8.71 ns |
| D115 | 11.2 ns | 12.4 ns | 12.5 ns | 12.4 ns | 14.1 ns |
| D153 | 8.95 ns | 15.9 ns | 15.9 ns | 16.2 ns | 16.4 ns |
| D230 | 36.6 ns | 36.3 ns | 36.2 ns | 32.1 ns | 31.8 ns |
| D307 | 48.5 ns | 50.3 ns | 43.6 ns | 43.6 ns | 42.8 ns |
| D462 | 74.5 ns | 86 ns | 90.4 ns | 86.1 ns | 93.4 ns |
| D616 | 85.8 ns | 90.7 ns | 95.7 ns | 98.2 ns | 107 ns |
| D924 | 110 ns | 115 ns | 100 ns | 101 ns | 102 ns |
| D1232 | 143 ns | 134 ns | 78.4 ns | 121 ns | 123 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,152.9 124.4,149.5 160.5,143.8 196.7,140.1 232.9,146.5 269.1,105.8 305.3,97.6 341.5,85.2 377.6,81.1 413.8,73.8 450.0,66.2 450.0,70.7 413.8,76.1 377.6,74.7 341.5,78.6 305.3,101.2 269.1,109.8 232.9,129.0 196.7,133.5 160.5,147.3 124.4,149.5 88.2,128.7 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,152.9 124.4,149.5 160.5,143.8 196.7,140.1 232.9,146.5 269.1,105.8 305.3,97.6 341.5,85.2 377.6,81.1 413.8,73.8 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,192.9 88.2,149.4 124.4,153.0 160.5,143.8 196.7,137.0 232.9,130.0 269.1,106.0 305.3,96.6 341.5,81.0 377.6,79.5 413.8,72.7 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.2 88.2,132.0 124.4,149.5 160.5,143.8 196.7,136.9 232.9,129.9 269.1,106.0 305.3,100.7 341.5,79.6 377.6,77.9 413.8,76.6 450.0,83.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,124.3 124.4,153.0 160.5,144.0 196.7,137.0 232.9,129.3 269.1,109.5 305.3,100.7 341.5,81.0 377.6,77.2 413.8,76.2 450.0,71.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,128.7 124.4,149.5 160.5,147.3 196.7,133.5 232.9,129.0 269.1,109.8 305.3,101.2 341.5,78.6 377.6,74.7 413.8,76.1 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.934 ns | 0.935 ns | 1.14 ns | 0.936 ns | 0.936 ns |
| D38 | 1.61 ns | 1.82 ns | 1.82 ns | 1.82 ns | 1.82 ns |
| D57 | 2.5 ns | 2.25 ns | 2.5 ns | 2.25 ns | 2.5 ns |
| D76 | 3.47 ns | 3.46 ns | 3.45 ns | 3.46 ns | 3.09 ns |
| D115 | 4.3 ns | 4.85 ns | 4.85 ns | 4.84 ns | 5.56 ns |
| D153 | 4.34 ns | 7.68 ns | 7.64 ns | 7.65 ns | 7.65 ns |
| D230 | 17.6 ns | 17.6 ns | 17.6 ns | 16.2 ns | 16.1 ns |
| D307 | 25.1 ns | 25.1 ns | 23.5 ns | 23.5 ns | 23.5 ns |
| D462 | 37.3 ns | 40.5 ns | 40.9 ns | 40.4 ns | 40.5 ns |
| D616 | 50.3 ns | 50.9 ns | 60.2 ns | 61.8 ns | 75.6 ns |
| D924 | 77.6 ns | 84.7 ns | 75.2 ns | 84.8 ns | 79 ns |
| D1232 | 106 ns | 83.7 ns | 63 ns | 95.6 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,133.0 196.7,128.3 232.9,128.1 269.1,97.7 305.3,90.0 341.5,81.4 377.6,74.9 413.8,65.5 450.0,58.7 450.0,58.7 413.8,65.1 377.6,66.1 341.5,79.6 305.3,91.5 269.1,99.7 232.9,115.8 196.7,122.7 160.5,135.5 124.4,140.1 88.2,147.0 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,133.0 196.7,128.3 232.9,128.1 269.1,97.7 305.3,90.0 341.5,81.4 377.6,74.9 413.8,65.5 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,147.0 124.4,142.3 160.5,133.1 196.7,125.7 232.9,115.7 269.1,97.7 305.3,90.0 341.5,79.6 377.6,74.7 413.8,63.6 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,146.9 124.4,140.1 160.5,133.1 196.7,125.7 232.9,115.9 269.1,97.7 305.3,91.4 341.5,79.4 377.6,71.0 413.8,66.2 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,142.4 160.5,133.1 196.7,125.8 232.9,115.8 269.1,99.6 305.3,91.4 341.5,79.7 377.6,70.5 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,140.1 160.5,135.5 196.7,122.7 232.9,115.8 269.1,99.7 305.3,91.5 341.5,79.6 377.6,66.1 413.8,65.1 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
