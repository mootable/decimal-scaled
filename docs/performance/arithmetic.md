# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.25 ns | 1.05 ns | 1.03 ns | 1.25 ns |
| D38 | 1.81 ns | 1.41 ns | 1.81 ns | 0.996 ns | 1.61 ns |
| D57 | 1.83 ns | 1.48 ns | 1.51 ns | 2.27 ns | 3.49 ns |
| D76 | 2.71 ns | 3.09 ns | 3.48 ns | 1.59 ns | 3.49 ns |
| D115 | 4.4 ns | 4.4 ns | 5 ns | 4.39 ns | 4.41 ns |
| D153 | 6.62 ns | 5.9 ns | 5.14 ns | 4.5 ns | 5.89 ns |
| D230 | 11.9 ns | 13.9 ns | 12 ns | 13.9 ns | 13.2 ns |
| D307 | 19.6 ns | 19.6 ns | 14.7 ns | 18.7 ns | 18.7 ns |
| D462 | 29.5 ns | 32.8 ns | 30.8 ns | 41.2 ns | 27.3 ns |
| D616 | 31.9 ns | 34.9 ns | 52.1 ns | 50.8 ns | 51.1 ns |
| D924 | 74.9 ns | 62.9 ns | 86.3 ns | 75.4 ns | 51.4 ns |
| D1232 | 70.7 ns | 95.4 ns | 63.7 ns | 61.6 ns | 84.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,141.8 88.2,126.1 124.4,125.9 160.5,114.5 196.7,100.4 232.9,88.6 269.1,71.7 305.3,57.2 341.5,45.4 377.6,43.1 413.8,18.4 450.0,20.0 450.0,14.8 413.8,29.2 377.6,29.4 341.5,47.6 305.3,58.5 269.1,68.7 232.9,92.0 196.7,100.4 160.5,107.1 124.4,107.1 88.2,129.6 52.0,137.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,141.8 88.2,126.1 124.4,125.9 160.5,114.5 196.7,100.4 232.9,88.6 269.1,71.7 305.3,57.2 341.5,45.4 377.6,43.1 413.8,18.4 450.0,20.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,137.0 88.2,133.4 124.4,131.9 160.5,110.7 196.7,100.4 232.9,91.9 269.1,67.0 305.3,57.2 341.5,42.3 377.6,40.4 413.8,23.4 450.0,11.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,126.1 124.4,131.3 160.5,107.2 196.7,96.8 232.9,95.9 269.1,71.5 305.3,65.4 341.5,44.1 377.6,28.9 413.8,14.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,142.4 88.2,143.4 124.4,119.6 160.5,129.9 196.7,100.5 232.9,99.8 269.1,67.1 305.3,58.5 341.5,35.7 377.6,29.6 413.8,18.2 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.0 88.2,129.6 124.4,107.1 160.5,107.1 196.7,100.4 232.9,92.0 269.1,68.7 305.3,58.5 341.5,47.6 377.6,29.4 413.8,29.2 450.0,14.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.45 ns | 5.42 ns | 7.04 ns | 8.98 ns | 7.84 ns |
| D38 | 12.7 ns | 9.81 ns | 14.3 ns | 61.1 ns | 59.8 ns |
| D57 | 32.7 ns | 33.7 ns | 47.2 ns | 107 ns | 115 ns |
| D76 | 31.4 ns | 61.3 ns | 85.1 ns | 72.4 ns | 144 ns |
| D115 | 56.4 ns | 84.2 ns | 113 ns | 177 ns | 221 ns |
| D153 | 69.3 ns | 108 ns | 121 ns | 227 ns | 317 ns |
| D230 | 87.3 ns | 154 ns | 195 ns | 385 ns | 587 ns |
| D307 | 158 ns | 262 ns | 338 ns | 575 ns | 870 ns |
| D462 | 213 ns | 438 ns | 563 ns | 1.06 µs | 1.17 µs |
| D616 | 141 ns | 482 ns | 1.09 µs | 1.83 µs | 2.4 µs |
| D924 | 404 ns | 853 ns | 2.27 µs | 2.68 µs | 3.44 µs |
| D1232 | 442 ns | 1.75 µs | 2.16 µs | 3.38 µs | 7.03 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.5 88.2,154.8 124.4,134.3 160.5,135.1 196.7,122.4 232.9,118.0 269.1,112.9 305.3,100.1 341.5,93.5 377.6,102.6 413.8,79.7 450.0,77.7 450.0,17.7 413.8,33.2 377.6,41.0 341.5,56.5 305.3,63.0 269.1,71.6 232.9,85.0 196.7,92.7 160.5,102.0 124.4,107.0 88.2,121.2 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.5 88.2,154.8 124.4,134.3 160.5,135.1 196.7,122.4 232.9,118.0 269.1,112.9 305.3,100.1 341.5,93.5 377.6,102.6 413.8,79.7 450.0,77.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.3 88.2,160.4 124.4,133.6 160.5,120.6 196.7,113.7 232.9,108.3 269.1,100.7 305.3,89.1 341.5,77.9 377.6,75.9 413.8,63.5 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.6 88.2,152.2 124.4,126.3 160.5,113.5 196.7,107.3 232.9,105.8 269.1,95.5 305.3,83.5 341.5,72.5 377.6,58.1 413.8,42.2 450.0,43.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.3 88.2,120.7 124.4,108.6 160.5,117.0 196.7,97.6 232.9,92.2 269.1,80.7 305.3,72.0 341.5,58.8 377.6,46.9 413.8,38.6 450.0,33.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,121.2 124.4,107.0 160.5,102.0 196.7,92.7 232.9,85.0 269.1,71.6 305.3,63.0 341.5,56.5 377.6,41.0 413.8,33.2 450.0,17.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 2.88 ns | 3.2 ns | 5.15 ns | 5.11 ns |
| D38 | 3.96 ns | 10.6 ns | 28.2 ns | 18.1 ns | 26.4 ns |
| D57 | 4.03 ns | 15.4 ns | 24.2 ns | 71.7 ns | 77.6 ns |
| D76 | 4.43 ns | 35 ns | 45.8 ns | 49.1 ns | 109 ns |
| D115 | 13.3 ns | 47.6 ns | 93.1 ns | 197 ns | 229 ns |
| D153 | 16.9 ns | 49.9 ns | 93.2 ns | 221 ns | 356 ns |
| D230 | 21.5 ns | 122 ns | 286 ns | 520 ns | 908 ns |
| D307 | 54.4 ns | 185 ns | 419 ns | 1.02 µs | 1.37 µs |
| D462 | 73.3 ns | 448 ns | 916 ns | 1.73 µs | 2.06 µs |
| D616 | 52.6 ns | 555 ns | 1.85 µs | 2.64 µs | 4.13 µs |
| D924 | 135 ns | 993 ns | 3.18 µs | 4.93 µs | 5.47 µs |
| D1232 | 153 ns | 2.2 µs | 2.54 µs | 6.09 µs | 12.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.1 124.4,185.8 160.5,184.1 196.7,165.1 232.9,160.9 269.1,156.7 305.3,140.6 341.5,135.4 377.6,141.2 413.8,124.8 450.0,122.6 450.0,46.7 413.8,60.5 377.6,65.4 341.5,77.5 305.3,84.6 269.1,91.7 232.9,107.9 196.7,115.6 160.5,128.6 124.4,134.4 88.2,153.2 52.0,181.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.1 124.4,185.8 160.5,184.1 196.7,165.1 232.9,160.9 269.1,156.7 305.3,140.6 341.5,135.4 377.6,141.2 413.8,124.8 450.0,122.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.7 88.2,168.9 124.4,162.5 160.5,148.2 196.7,142.9 232.9,142.1 269.1,126.5 305.3,119.3 341.5,103.9 377.6,100.2 413.8,90.1 450.0,76.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,152.0 124.4,154.7 160.5,143.6 196.7,131.2 232.9,131.2 269.1,111.8 305.3,105.1 341.5,91.5 377.6,79.3 413.8,69.9 450.0,73.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.5 88.2,159.7 124.4,135.8 160.5,142.4 196.7,118.2 232.9,116.2 269.1,101.4 305.3,89.7 341.5,80.5 377.6,73.1 413.8,62.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.6 88.2,153.2 124.4,134.4 160.5,128.6 196.7,115.6 232.9,107.9 269.1,91.7 305.3,84.6 341.5,77.5 377.6,65.4 413.8,60.5 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.622 ns | 0.703 ns | 0.346 ns | 0.622 ns |
| D38 | 1.45 ns | 1.12 ns | 1.45 ns | 0.938 ns | 1.32 ns |
| D57 | 1.53 ns | 1.29 ns | 1.38 ns | 1.87 ns | 1.74 ns |
| D76 | 1.68 ns | 2.19 ns | 2.17 ns | 1.39 ns | 2.63 ns |
| D115 | 2.85 ns | 2.85 ns | 3.17 ns | 3.28 ns | 3.28 ns |
| D153 | 4.22 ns | 3.79 ns | 3.57 ns | 3.48 ns | 4.49 ns |
| D230 | 5.16 ns | 5.86 ns | 5.62 ns | 7.16 ns | 5.36 ns |
| D307 | 12.3 ns | 12.4 ns | 7.82 ns | 11.1 ns | 11.1 ns |
| D462 | 15.1 ns | 17 ns | 14.8 ns | 15.3 ns | 14.3 ns |
| D616 | 11 ns | 16.1 ns | 22 ns | 20.4 ns | 22 ns |
| D924 | 54.9 ns | 61.5 ns | 84.7 ns | 75 ns | 47.5 ns |
| D1232 | 32.7 ns | 61.5 ns | 37.8 ns | 36.5 ns | 50.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.7 124.4,131.0 160.5,128.3 196.7,113.0 232.9,101.7 269.1,95.8 305.3,70.8 341.5,64.8 377.6,73.8 413.8,27.4 450.0,42.4 450.0,29.7 413.8,31.6 377.6,53.9 341.5,66.2 305.3,73.7 269.1,94.7 232.9,99.9 196.7,109.0 160.5,115.3 124.4,127.3 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.7 124.4,131.0 160.5,128.3 196.7,113.0 232.9,101.7 269.1,95.8 305.3,70.8 341.5,64.8 377.6,73.8 413.8,27.4 450.0,42.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,140.0 124.4,135.8 160.5,120.7 196.7,113.0 232.9,104.8 269.1,92.2 305.3,70.5 341.5,61.2 377.6,62.9 413.8,24.1 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,134.1 160.5,120.9 196.7,110.0 232.9,106.5 269.1,93.4 305.3,83.8 341.5,65.3 377.6,53.9 413.8,14.8 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,145.2 124.4,125.2 160.5,133.9 196.7,108.9 232.9,107.2 269.1,86.3 305.3,73.7 341.5,64.4 377.6,56.0 413.8,18.3 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,115.3 196.7,109.0 232.9,99.9 269.1,94.7 305.3,73.7 341.5,66.2 377.6,53.9 413.8,31.6 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.87 ns | 2.11 ns | 2.88 ns | 2.18 ns |
| D38 | 8.45 ns | 6.35 ns | 14.5 ns | 6.85 ns | 13.1 ns |
| D57 | 5.09 ns | 4.18 ns | 4.43 ns | 7.16 ns | 8.09 ns |
| D76 | 7.63 ns | 8.72 ns | 9.6 ns | 4.42 ns | 9.65 ns |
| D115 | 12.7 ns | 12.4 ns | 14.9 ns | 12.4 ns | 12.4 ns |
| D153 | 20.7 ns | 16 ns | 15.5 ns | 11.8 ns | 16.6 ns |
| D230 | 28.3 ns | 32.3 ns | 28.1 ns | 32 ns | 32.6 ns |
| D307 | 48.8 ns | 48 ns | 30.2 ns | 42.6 ns | 42.6 ns |
| D462 | 73.8 ns | 87.3 ns | 57.5 ns | 75.3 ns | 65.4 ns |
| D616 | 45.1 ns | 71.6 ns | 94.5 ns | 87.5 ns | 90.7 ns |
| D924 | 112 ns | 97.8 ns | 110 ns | 108 ns | 58.7 ns |
| D1232 | 102 ns | 131 ns | 71 ns | 69 ns | 81.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,148.2 124.4,162.9 160.5,151.2 196.7,136.3 232.9,122.2 269.1,113.2 305.3,97.4 341.5,85.4 377.6,99.7 413.8,73.3 450.0,76.0 450.0,82.6 413.8,92.1 377.6,79.5 341.5,89.0 305.3,101.4 269.1,109.1 232.9,128.7 196.7,137.0 160.5,144.4 124.4,149.5 88.2,135.4 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,148.2 124.4,162.9 160.5,151.2 196.7,136.3 232.9,122.2 269.1,113.2 305.3,97.4 341.5,85.4 377.6,99.7 413.8,73.3 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,156.5 124.4,168.6 160.5,147.3 196.7,137.1 232.9,129.8 269.1,109.4 305.3,97.9 341.5,80.6 377.6,86.3 413.8,77.3 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,132.6 124.4,166.9 160.5,144.5 196.7,131.8 232.9,130.6 269.1,113.4 305.3,111.3 341.5,92.7 377.6,78.3 413.8,74.0 450.0,86.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,154.3 124.4,153.0 160.5,167.0 196.7,137.0 232.9,138.6 269.1,109.7 305.3,101.4 341.5,84.9 377.6,80.5 413.8,74.4 450.0,87.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,135.4 124.4,149.5 160.5,144.4 196.7,137.0 232.9,128.7 269.1,109.1 305.3,101.4 341.5,89.0 377.6,79.5 413.8,92.1 450.0,82.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 0.937 ns | 1.05 ns | 1.06 ns | 0.936 ns |
| D38 | 1.81 ns | 1.41 ns | 1.81 ns | 0.906 ns | 1.61 ns |
| D57 | 1.9 ns | 1.53 ns | 1.55 ns | 2.27 ns | 3.68 ns |
| D76 | 2.69 ns | 3.1 ns | 3.46 ns | 2.16 ns | 3.46 ns |
| D115 | 4.85 ns | 4.84 ns | 5.56 ns | 4.84 ns | 4.83 ns |
| D153 | 8.46 ns | 7.62 ns | 6.55 ns | 6.37 ns | 7.64 ns |
| D230 | 13.7 ns | 16.2 ns | 13.7 ns | 16.1 ns | 14.5 ns |
| D307 | 25.2 ns | 25.1 ns | 18.1 ns | 23.6 ns | 23.4 ns |
| D462 | 37.4 ns | 42.3 ns | 36.6 ns | 49.5 ns | 34.3 ns |
| D616 | 32 ns | 35.2 ns | 51.2 ns | 51.1 ns | 48.8 ns |
| D924 | 75.9 ns | 68.6 ns | 85.8 ns | 75.4 ns | 56.6 ns |
| D1232 | 77.7 ns | 96.9 ns | 62.5 ns | 68.3 ns | 94.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,141.7 88.2,126.2 124.4,124.8 160.5,114.7 196.7,97.6 232.9,81.5 269.1,67.5 305.3,49.9 341.5,38.4 377.6,43.0 413.8,18.0 450.0,17.3 450.0,11.5 413.8,26.5 377.6,30.8 341.5,41.0 305.3,52.0 269.1,66.0 232.9,84.5 196.7,97.7 160.5,107.4 124.4,105.6 88.2,129.5 52.0,145.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,141.7 88.2,126.2 124.4,124.8 160.5,114.7 196.7,97.6 232.9,81.5 269.1,67.5 305.3,49.9 341.5,38.4 377.6,43.0 413.8,18.0 450.0,17.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.2 88.2,133.4 124.4,131.1 160.5,110.5 196.7,97.7 232.9,84.5 269.1,62.7 305.3,50.0 341.5,34.9 377.6,40.2 413.8,20.9 450.0,10.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,126.1 124.4,130.6 160.5,107.4 196.7,93.7 232.9,88.9 269.1,67.5 305.3,59.6 341.5,39.1 377.6,29.4 413.8,14.4 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.7 88.2,146.2 124.4,119.6 160.5,121.0 196.7,97.7 232.9,89.7 269.1,62.8 305.3,51.8 341.5,30.3 377.6,29.5 413.8,18.2 450.0,21.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.2 88.2,129.5 124.4,105.6 160.5,107.4 196.7,97.7 232.9,84.5 269.1,66.0 305.3,52.0 341.5,41.0 377.6,30.8 413.8,26.5 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
