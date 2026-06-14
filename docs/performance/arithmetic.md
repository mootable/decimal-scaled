# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.934 ns | 1.06 ns | 0.935 ns | 1.05 ns | 1.06 ns |
| D38 | 1.61 ns | 1.82 ns | 1.61 ns | 1.83 ns | 1.62 ns |
| D57 | 2.5 ns | 2.49 ns | 2.26 ns | 2.25 ns | 2.25 ns |
| D76 | 1.59 ns | 3.08 ns | 3.5 ns | 3.09 ns | 3.48 ns |
| D115 | 4.99 ns | 4.97 ns | 4.4 ns | 4.4 ns | 4.4 ns |
| D153 | 5.95 ns | 5.94 ns | 5.94 ns | 5.94 ns | 5.95 ns |
| D230 | 13.8 ns | 12.1 ns | 13.8 ns | 15.4 ns | 13.9 ns |
| D307 | 19.6 ns | 19.6 ns | 18.5 ns | 18.5 ns | 19.7 ns |
| D462 | 33.2 ns | 33 ns | 44.6 ns | 30.3 ns | 28.9 ns |
| D616 | 45.2 ns | 78 ns | 61.4 ns | 58.2 ns | 51.2 ns |
| D924 | 74.6 ns | 75 ns | 84.8 ns | 87.5 ns | 98.2 ns |
| D1232 | 108 ns | 107 ns | 71 ns | 107 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,150.0 196.7,125.1 232.9,121.3 269.1,102.9 305.3,95.4 341.5,83.9 377.6,77.2 413.8,66.4 450.0,58.4 450.0,58.6 413.8,60.4 377.6,74.6 341.5,87.0 305.3,95.3 269.1,102.9 232.9,121.3 196.7,127.8 160.5,132.9 124.4,142.4 88.2,149.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,150.0 196.7,125.1 232.9,121.3 269.1,102.9 305.3,95.4 341.5,83.9 377.6,77.2 413.8,66.4 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.2 160.5,135.5 196.7,125.2 232.9,121.3 269.1,105.8 305.3,95.4 341.5,84.1 377.6,65.4 413.8,66.2 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.3 160.5,132.8 196.7,127.8 232.9,121.3 269.1,102.9 305.3,96.6 341.5,77.6 377.6,70.6 413.8,63.6 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.4 160.5,135.5 196.7,127.8 232.9,121.3 269.1,100.7 305.3,96.6 341.5,86.0 377.6,71.7 413.8,62.9 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,142.4 160.5,132.9 196.7,127.8 232.9,121.3 269.1,102.9 305.3,95.3 341.5,87.0 377.6,74.6 413.8,60.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.96 ns | 6.32 ns | 5.7 ns | 9.22 ns | 9.33 ns |
| D38 | 9.55 ns | 12.4 ns | 15.2 ns | 811 ns | 946 ns |
| D57 | 35.3 ns | 52 ns | 68.8 ns | 100 ns | 102 ns |
| D76 | 26.8 ns | 60.3 ns | 85.5 ns | 104 ns | 137 ns |
| D115 | 56.9 ns | 84.6 ns | 104 ns | 172 ns | 209 ns |
| D153 | 66.3 ns | 109 ns | 143 ns | 222 ns | 299 ns |
| D230 | 95.9 ns | 136 ns | 227 ns | 401 ns | 535 ns |
| D307 | 148 ns | 239 ns | 357 ns | 559 ns | 927 ns |
| D462 | 231 ns | 438 ns | 720 ns | 1.03 µs | 1.34 µs |
| D616 | 261 ns | 664 ns | 1.1 µs | 1.92 µs | 2.4 µs |
| D924 | 362 ns | 1.06 µs | 2.23 µs | 2.81 µs | 4.65 µs |
| D1232 | 533 ns | 1.87 µs | 3.06 µs | 4.63 µs | 7.83 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.3 88.2,161.0 124.4,132.6 160.5,138.6 196.7,122.2 232.9,118.9 269.1,110.9 305.3,101.5 341.5,91.8 377.6,89.1 413.8,82.1 450.0,73.7 450.0,15.3 413.8,26.6 377.6,41.0 341.5,53.6 305.3,61.6 269.1,73.6 232.9,86.2 196.7,94.0 160.5,103.1 124.4,109.5 88.2,61.2 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.3 88.2,161.0 124.4,132.6 160.5,138.6 196.7,122.2 232.9,118.9 269.1,110.9 305.3,101.5 341.5,91.8 377.6,89.1 413.8,82.1 450.0,73.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,155.3 124.4,124.2 160.5,121.0 196.7,113.6 232.9,108.2 269.1,103.4 305.3,91.0 341.5,77.9 377.6,68.9 413.8,58.7 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,151.0 124.4,118.1 160.5,113.4 196.7,109.1 232.9,102.2 269.1,92.2 305.3,82.4 341.5,67.1 377.6,57.9 413.8,42.6 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,64.6 124.4,109.9 160.5,109.1 196.7,98.3 232.9,92.7 269.1,79.8 305.3,72.6 341.5,59.3 377.6,45.8 413.8,37.6 450.0,26.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,61.2 124.4,109.5 160.5,103.1 196.7,94.0 232.9,86.2 269.1,73.6 305.3,61.6 341.5,53.6 377.6,41.0 413.8,26.6 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.886 ns | 3.06 ns | 3.1 ns | 5.02 ns | 4.98 ns |
| D38 | 3.5 ns | 13.7 ns | 27.2 ns | 30.4 ns | 28.4 ns |
| D57 | 4.22 ns | 24.4 ns | 32.9 ns | 71.5 ns | 71.8 ns |
| D76 | 3.87 ns | 34.2 ns | 43.6 ns | 80.1 ns | 107 ns |
| D115 | 13.6 ns | 57.3 ns | 93.7 ns | 194 ns | 226 ns |
| D153 | 18.7 ns | 52.7 ns | 112 ns | 234 ns | 353 ns |
| D230 | 27.2 ns | 102 ns | 337 ns | 574 ns | 969 ns |
| D307 | 55 ns | 188 ns | 459 ns | 1.06 µs | 1.45 µs |
| D462 | 87.4 ns | 450 ns | 1.36 µs | 1.74 µs | 2.44 µs |
| D616 | 91.9 ns | 726 ns | 1.87 µs | 2.92 µs | 4.17 µs |
| D924 | 160 ns | 1.47 µs | 3.18 µs | 5.4 µs | 8.29 µs |
| D1232 | 200 ns | 2.39 µs | 4.57 µs | 8.93 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,157.1 196.7,138.8 232.9,134.3 269.1,128.9 305.3,118.7 341.5,112.0 377.6,111.2 413.8,103.2 450.0,100.0 450.0,38.3 413.8,46.1 377.6,56.0 341.5,63.7 305.3,71.3 269.1,77.1 232.9,91.7 196.7,98.2 160.5,109.0 124.4,114.8 88.2,128.2 52.0,153.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,157.1 196.7,138.8 232.9,134.3 269.1,128.9 305.3,118.7 341.5,112.0 377.6,111.2 413.8,103.2 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.5 88.2,138.7 124.4,130.4 160.5,125.5 196.7,118.1 232.9,119.3 269.1,109.7 305.3,100.9 341.5,88.2 377.6,81.3 413.8,71.1 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,128.8 124.4,126.1 160.5,122.0 196.7,110.9 232.9,108.3 269.1,92.4 305.3,87.9 341.5,72.2 377.6,67.6 413.8,59.9 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,127.3 124.4,114.8 160.5,113.2 196.7,100.4 232.9,97.7 269.1,84.7 305.3,75.8 341.5,68.6 377.6,61.1 413.8,52.2 450.0,45.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,128.2 124.4,114.8 160.5,109.0 196.7,98.2 232.9,91.7 269.1,77.1 305.3,71.3 341.5,63.7 377.6,56.0 413.8,46.1 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.702 ns | 0.622 ns | 0.703 ns | 0.703 ns |
| D38 | 1.33 ns | 1.45 ns | 1.33 ns | 1.45 ns | 1.32 ns |
| D57 | 1.74 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.87 ns |
| D76 | 1.33 ns | 2.1 ns | 2.16 ns | 2.1 ns | 2.63 ns |
| D115 | 3.17 ns | 3.17 ns | 2.86 ns | 3.25 ns | 3.25 ns |
| D153 | 3.81 ns | 3.82 ns | 4.29 ns | 4.3 ns | 4.29 ns |
| D230 | 5.86 ns | 4.29 ns | 7.16 ns | 7.24 ns | 7.16 ns |
| D307 | 12.3 ns | 12.5 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 17.5 ns | 16.7 ns | 28.7 ns | 15.3 ns | 14.9 ns |
| D616 | 19 ns | 35.1 ns | 21.8 ns | 21.9 ns | 22 ns |
| D924 | 54.9 ns | 75.5 ns | 84.8 ns | 86.7 ns | 94 ns |
| D1232 | 54.4 ns | 69.9 ns | 44 ns | 69.7 ns | 69.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,135.0 196.7,110.0 232.9,104.6 269.1,92.1 305.3,70.8 341.5,60.4 377.6,58.1 413.8,27.3 450.0,27.6 450.0,20.4 413.8,11.8 377.6,53.9 341.5,65.1 305.3,70.2 269.1,86.3 232.9,101.1 196.7,109.2 160.5,115.3 124.4,125.2 88.2,135.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,135.0 196.7,110.0 232.9,104.6 269.1,92.1 305.3,70.8 341.5,60.4 377.6,58.1 413.8,27.3 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.6 88.2,132.6 124.4,127.3 160.5,121.8 196.7,110.0 232.9,104.5 269.1,101.2 305.3,70.2 341.5,61.9 377.6,40.3 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,125.2 160.5,121.0 196.7,112.9 232.9,101.2 269.1,86.3 305.3,73.7 341.5,46.1 377.6,54.1 413.8,14.8 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,125.2 160.5,121.9 196.7,109.2 232.9,101.1 269.1,86.0 305.3,73.7 341.5,64.4 377.6,53.9 413.8,14.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,115.3 196.7,109.2 232.9,101.1 269.1,86.3 305.3,70.2 341.5,65.1 377.6,53.9 413.8,11.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 1.86 ns | 2.46 ns | 2.46 ns |
| D38 | 7.18 ns | 8.37 ns | 12.4 ns | 19.4 ns | 13.1 ns |
| D57 | 8.09 ns | 8.09 ns | 7.16 ns | 7.16 ns | 7.16 ns |
| D76 | 4.9 ns | 8.71 ns | 9.6 ns | 8.72 ns | 9.83 ns |
| D115 | 14.4 ns | 14.1 ns | 12.4 ns | 12.7 ns | 12.4 ns |
| D153 | 17.2 ns | 15.9 ns | 15.9 ns | 16.2 ns | 16.4 ns |
| D230 | 32.2 ns | 23.6 ns | 32.3 ns | 36.3 ns | 32.1 ns |
| D307 | 48.5 ns | 50.3 ns | 45 ns | 43.1 ns | 48.2 ns |
| D462 | 89 ns | 85.5 ns | 102 ns | 83.3 ns | 81.9 ns |
| D616 | 84.6 ns | 116 ns | 95.9 ns | 97.7 ns | 93.1 ns |
| D924 | 108 ns | 104 ns | 111 ns | 119 ns | 111 ns |
| D1232 | 143 ns | 137 ns | 93.4 ns | 124 ns | 126 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,152.9 124.4,149.5 160.5,164.0 196.7,132.7 232.9,127.7 269.1,109.5 305.3,97.6 341.5,80.1 377.6,81.5 413.8,74.3 450.0,66.2 450.0,70.0 413.8,73.5 377.6,78.7 341.5,82.5 305.3,97.8 269.1,109.5 232.9,129.0 196.7,137.0 160.5,143.8 124.4,153.0 88.2,135.5 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,152.9 124.4,149.5 160.5,164.0 196.7,132.7 232.9,127.7 269.1,109.5 305.3,97.6 341.5,80.1 377.6,81.5 413.8,74.3 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,148.5 124.4,149.5 160.5,147.3 196.7,133.5 232.9,129.8 269.1,118.5 305.3,96.6 341.5,81.2 377.6,72.4 413.8,75.6 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.0 88.2,137.1 124.4,153.0 160.5,144.5 196.7,137.0 232.9,129.9 269.1,109.4 305.3,99.8 341.5,76.2 377.6,77.9 413.8,73.7 450.0,78.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,124.2 124.4,153.0 160.5,147.3 196.7,136.4 232.9,129.4 269.1,106.0 305.3,101.0 341.5,82.0 377.6,77.3 413.8,71.7 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,135.5 124.4,153.0 160.5,143.8 196.7,137.0 232.9,129.0 269.1,109.5 305.3,97.8 341.5,82.5 377.6,78.7 413.8,73.5 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 1.05 ns | 0.935 ns | 1.05 ns | 1.05 ns |
| D38 | 1.6 ns | 1.82 ns | 1.61 ns | 1.81 ns | 1.61 ns |
| D57 | 2.5 ns | 2.5 ns | 2.25 ns | 2.25 ns | 2.25 ns |
| D76 | 1.83 ns | 3.1 ns | 3.45 ns | 3.1 ns | 3.46 ns |
| D115 | 5.55 ns | 5.56 ns | 4.84 ns | 4.84 ns | 4.84 ns |
| D153 | 7.61 ns | 7.62 ns | 7.63 ns | 7.63 ns | 7.64 ns |
| D230 | 16.1 ns | 13.7 ns | 16.1 ns | 17.7 ns | 16.1 ns |
| D307 | 25.2 ns | 25.1 ns | 23.5 ns | 23.7 ns | 25.2 ns |
| D462 | 43.1 ns | 40.6 ns | 47.8 ns | 38.9 ns | 37.1 ns |
| D616 | 45.9 ns | 81.1 ns | 62.8 ns | 60 ns | 48.9 ns |
| D924 | 74.9 ns | 75.6 ns | 84.8 ns | 86.7 ns | 98.4 ns |
| D1232 | 106 ns | 106 ns | 77.8 ns | 106 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.8 124.4,140.1 160.5,146.9 196.7,122.8 232.9,115.9 269.1,99.7 305.3,89.9 341.5,78.3 377.6,76.9 413.8,66.3 450.0,58.7 450.0,58.7 413.8,60.4 377.6,75.5 341.5,81.5 305.3,89.9 269.1,99.6 232.9,115.8 196.7,125.7 160.5,133.0 124.4,142.4 88.2,149.7 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.8 124.4,140.1 160.5,146.9 196.7,122.8 232.9,115.9 269.1,99.7 305.3,89.9 341.5,78.3 377.6,76.9 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,135.4 196.7,122.8 232.9,115.9 269.1,103.2 305.3,90.0 341.5,79.5 377.6,64.6 413.8,66.1 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.4 160.5,133.1 196.7,125.8 232.9,115.9 269.1,99.6 305.3,91.4 341.5,76.0 377.6,70.1 413.8,63.6 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,142.4 160.5,135.5 196.7,125.8 232.9,115.9 269.1,97.7 305.3,91.3 341.5,80.5 377.6,71.1 413.8,63.1 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,142.4 160.5,133.0 196.7,125.7 232.9,115.8 269.1,99.6 305.3,89.9 341.5,81.5 377.6,75.5 413.8,60.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
