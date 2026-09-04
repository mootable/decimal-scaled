# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.941 ns | 0.82 ns | 0.944 ns | 0.944 ns | 0.818 ns |
| D38 | 1.62 ns | 1.41 ns | 1.81 ns | 1.61 ns | 1.61 ns |
| D57 | 2.5 ns | 1.47 ns | 2.29 ns | 1.71 ns | 2.27 ns |
| D76 | 1.79 ns | 2.7 ns | 3.08 ns | 3.09 ns | 3.09 ns |
| D115 | 4.4 ns | 5.01 ns | 4.98 ns | 4.4 ns | 4.4 ns |
| D153 | 3.89 ns | 6.63 ns | 3.5 ns | 3.78 ns | 5.89 ns |
| D230 | 15.4 ns | 12.1 ns | 13.9 ns | 13.9 ns | 13.9 ns |
| D307 | 18.5 ns | 18.6 ns | 19.6 ns | 18.5 ns | 18.6 ns |
| D462 | 25.8 ns | 26.6 ns | 40.3 ns | 28.6 ns | 28.6 ns |
| D616 | 34 ns | 51.3 ns | 45 ns | 56.5 ns | 38.6 ns |
| D924 | 84.9 ns | 74.7 ns | 51.5 ns | 74.9 ns | 84.9 ns |
| D1232 | 62.7 ns | 76.6 ns | 95 ns | 95 ns | 84.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,145.1 88.2,129.4 124.4,116.8 160.5,126.5 196.7,100.4 232.9,104.0 269.1,64.2 305.3,58.8 341.5,49.3 377.6,41.2 413.8,14.8 450.0,23.5 450.0,15.0 413.8,14.8 377.6,37.6 341.5,46.2 305.3,58.7 269.1,67.2 232.9,92.0 196.7,100.5 160.5,110.7 124.4,119.6 88.2,129.5 52.0,149.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,145.1 88.2,129.4 124.4,116.8 160.5,126.5 196.7,100.4 232.9,104.0 269.1,64.2 305.3,58.8 341.5,49.3 377.6,41.2 413.8,14.8 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,149.1 88.2,133.4 124.4,132.2 160.5,114.6 196.7,96.7 232.9,88.6 269.1,71.1 305.3,58.7 341.5,48.3 377.6,29.3 413.8,18.4 450.0,17.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.0 88.2,126.1 124.4,119.4 160.5,110.8 196.7,96.9 232.9,107.0 269.1,67.2 305.3,57.2 341.5,36.3 377.6,33.1 413.8,29.2 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.0 88.2,129.5 124.4,127.9 160.5,110.7 196.7,100.4 232.9,104.8 269.1,67.2 305.3,58.8 341.5,46.3 377.6,26.6 413.8,18.4 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,149.1 88.2,129.5 124.4,119.6 160.5,110.7 196.7,100.5 232.9,92.0 269.1,67.2 305.3,58.7 341.5,46.2 377.6,37.6 413.8,14.8 450.0,15.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.78 ns | 4.82 ns | 5.99 ns | 8.18 ns | 6.95 ns |
| D38 | 9.78 ns | 9.81 ns | 14.3 ns | 59.2 ns | 59.5 ns |
| D57 | 21.9 ns | 22 ns | 67.8 ns | 92.6 ns | 113 ns |
| D76 | 20.8 ns | 51.7 ns | 75.4 ns | 115 ns | 142 ns |
| D115 | 43.2 ns | 88.6 ns | 112 ns | 187 ns | 228 ns |
| D153 | 37.3 ns | 115 ns | 88.2 ns | 179 ns | 333 ns |
| D230 | 94.6 ns | 151 ns | 231 ns | 395 ns | 575 ns |
| D307 | 117 ns | 223 ns | 399 ns | 585 ns | 877 ns |
| D462 | 143 ns | 263 ns | 684 ns | 1.05 µs | 1.33 µs |
| D616 | 167 ns | 598 ns | 1.03 µs | 1.98 µs | 2.38 µs |
| D924 | 383 ns | 1.08 µs | 1.27 µs | 2.55 µs | 4.68 µs |
| D1232 | 267 ns | 1.48 µs | 3.44 µs | 4.15 µs | 6.06 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.9 88.2,160.5 124.4,143.0 160.5,144.1 196.7,128.2 232.9,131.4 269.1,111.2 305.3,106.6 341.5,102.3 377.6,98.9 413.8,80.8 450.0,88.7 450.0,20.9 413.8,26.5 377.6,41.1 341.5,53.8 305.3,62.8 269.1,72.0 232.9,83.9 196.7,92.1 160.5,102.3 124.4,107.4 88.2,121.3 52.0,167.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.9 88.2,160.5 124.4,143.0 160.5,144.1 196.7,128.2 232.9,131.4 269.1,111.2 305.3,106.6 341.5,102.3 377.6,98.9 413.8,80.8 450.0,88.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.9 88.2,160.4 124.4,142.8 160.5,124.3 196.7,112.6 232.9,106.9 269.1,101.1 305.3,92.6 341.5,89.0 377.6,71.2 413.8,58.3 450.0,51.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.1 88.2,152.2 124.4,118.4 160.5,116.1 196.7,107.5 232.9,112.7 269.1,91.8 305.3,80.0 341.5,68.3 377.6,59.3 413.8,54.8 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.4 88.2,121.4 124.4,111.7 160.5,106.9 196.7,96.4 232.9,97.4 269.1,80.2 305.3,71.6 341.5,59.0 377.6,45.2 413.8,39.7 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.9 88.2,121.3 124.4,107.4 160.5,102.3 196.7,92.1 232.9,83.9 269.1,72.0 305.3,62.8 341.5,53.8 377.6,41.1 413.8,26.5 450.0,20.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.886 ns | 2.38 ns | 3.11 ns | 4.93 ns | 3.84 ns |
| D38 | 3.51 ns | 10.6 ns | 27.8 ns | 26.6 ns | 26.4 ns |
| D57 | 4.22 ns | 15.1 ns | 32.9 ns | 62.1 ns | 71.6 ns |
| D76 | 4.48 ns | 29.9 ns | 42.2 ns | 78.8 ns | 102 ns |
| D115 | 13.5 ns | 54.7 ns | 93.4 ns | 196 ns | 232 ns |
| D153 | 11.7 ns | 55.2 ns | 67.2 ns | 181 ns | 353 ns |
| D230 | 28.3 ns | 101 ns | 338 ns | 519 ns | 997 ns |
| D307 | 44.4 ns | 167 ns | 508 ns | 1.04 µs | 1.41 µs |
| D462 | 52.6 ns | 269 ns | 1.24 µs | 1.76 µs | 2.44 µs |
| D616 | 65 ns | 684 ns | 1.77 µs | 2.96 µs | 3.81 µs |
| D924 | 161 ns | 1.49 µs | 2.01 µs | 5.06 µs | 8.3 µs |
| D1232 | 107 ns | 1.8 µs | 4.68 µs | 8.29 µs | 11 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,155.0 196.7,139.0 232.9,141.1 269.1,128.3 305.3,121.7 341.5,119.3 377.6,116.2 413.8,103.1 450.0,109.1 450.0,41.9 413.8,46.0 377.6,57.3 341.5,63.7 305.3,71.7 269.1,76.7 232.9,91.7 196.7,97.8 160.5,109.7 124.4,114.8 88.2,129.3 52.0,157.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,155.8 160.5,155.0 196.7,139.0 232.9,141.1 269.1,128.3 305.3,121.7 341.5,119.3 377.6,116.2 413.8,103.1 450.0,109.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.1 88.2,142.4 124.4,137.3 160.5,127.5 196.7,118.7 232.9,118.6 269.1,109.9 305.3,102.6 341.5,95.7 377.6,82.2 413.8,70.9 450.0,68.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,128.5 124.4,126.1 160.5,122.5 196.7,111.0 232.9,115.8 269.1,92.4 305.3,86.5 341.5,73.6 377.6,68.4 413.8,66.5 450.0,54.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,129.2 124.4,116.9 160.5,113.4 196.7,100.2 232.9,101.4 269.1,86.2 305.3,76.1 341.5,68.5 377.6,60.9 413.8,53.2 450.0,46.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,129.3 124.4,114.8 160.5,109.7 196.7,97.8 232.9,91.7 269.1,76.7 305.3,71.7 341.5,63.7 377.6,57.3 413.8,46.0 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.545 ns | 0.622 ns | 0.622 ns | 0.545 ns |
| D38 | 1.33 ns | 1.12 ns | 1.45 ns | 1.32 ns | 1.33 ns |
| D57 | 1.74 ns | 1.26 ns | 1.68 ns | 1.46 ns | 1.68 ns |
| D76 | 1.45 ns | 1.68 ns | 2.09 ns | 2.09 ns | 2.48 ns |
| D115 | 2.86 ns | 3.17 ns | 3.17 ns | 3.25 ns | 3.25 ns |
| D153 | 2.79 ns | 4.22 ns | 2.67 ns | 2.8 ns | 4.3 ns |
| D230 | 6.65 ns | 4.36 ns | 7.16 ns | 7.16 ns | 7.16 ns |
| D307 | 10.9 ns | 11 ns | 12.4 ns | 11 ns | 11 ns |
| D462 | 11.7 ns | 12.7 ns | 14.9 ns | 15 ns | 14.9 ns |
| D616 | 14.4 ns | 20.1 ns | 19.9 ns | 21.9 ns | 15.1 ns |
| D924 | 63.2 ns | 75.7 ns | 47.8 ns | 76.6 ns | 84.9 ns |
| D1232 | 24.9 ns | 51 ns | 61.4 ns | 61.5 ns | 52.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,132.7 196.7,112.9 232.9,113.7 269.1,88.5 305.3,74.2 341.5,72.0 377.6,66.2 413.8,23.3 450.0,50.3 450.0,28.8 413.8,14.7 377.6,64.7 341.5,65.1 305.3,73.9 269.1,86.3 232.9,101.1 196.7,109.2 160.5,117.0 124.4,128.3 88.2,135.1 52.0,160.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,132.7 196.7,112.9 232.9,113.7 269.1,88.5 305.3,74.2 341.5,72.0 377.6,66.2 413.8,23.3 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.9 88.2,140.0 124.4,136.6 160.5,128.3 196.7,110.0 232.9,101.6 269.1,100.7 305.3,73.9 341.5,69.9 377.6,56.4 413.8,18.0 450.0,29.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,128.3 160.5,122.0 196.7,110.0 232.9,114.9 269.1,86.4 305.3,70.5 341.5,65.1 377.6,56.8 413.8,31.4 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,132.3 160.5,122.0 196.7,109.2 232.9,113.5 269.1,86.3 305.3,73.9 341.5,64.9 377.6,53.9 413.8,17.7 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,135.1 124.4,128.3 160.5,117.0 196.7,109.2 232.9,101.1 269.1,86.3 305.3,73.9 341.5,65.1 377.6,64.7 413.8,14.7 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.64 ns | 1.87 ns | 2.18 ns | 1.91 ns |
| D38 | 3.43 ns | 2.75 ns | 3.55 ns | 3.19 ns | 3.2 ns |
| D57 | 8.09 ns | 4.57 ns | 7.17 ns | 4.3 ns | 7.16 ns |
| D76 | 5.52 ns | 7.37 ns | 8.71 ns | 8.72 ns | 8.41 ns |
| D115 | 12.7 ns | 14.4 ns | 14.4 ns | 12.8 ns | 12.8 ns |
| D153 | 10.1 ns | 20 ns | 9.27 ns | 10.9 ns | 17.8 ns |
| D230 | 36.5 ns | 24.1 ns | 32.1 ns | 32.2 ns | 32.1 ns |
| D307 | 41.9 ns | 40.4 ns | 47.9 ns | 42.7 ns | 42.6 ns |
| D462 | 60.5 ns | 49.2 ns | 74.5 ns | 80.1 ns | 71 ns |
| D616 | 70.4 ns | 88.8 ns | 78.9 ns | 96.4 ns | 61.2 ns |
| D924 | 109 ns | 106 ns | 62.9 ns | 84.4 ns | 87.9 ns |
| D1232 | 79.5 ns | 88.6 ns | 121 ns | 115 ns | 101 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,174.3 124.4,149.5 160.5,160.5 196.7,136.3 232.9,143.0 269.1,105.9 305.3,101.9 341.5,91.2 377.6,86.8 413.8,74.1 450.0,83.3 450.0,76.5 413.8,80.4 377.6,90.9 341.5,86.6 305.3,101.4 269.1,109.5 232.9,126.7 196.7,136.3 160.5,148.3 124.4,153.0 88.2,176.3 52.0,191.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,174.3 124.4,149.5 160.5,160.5 196.7,136.3 232.9,143.0 269.1,105.9 305.3,101.9 341.5,91.2 377.6,86.8 413.8,74.1 450.0,83.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,195.8 88.2,180.7 124.4,166.0 160.5,152.2 196.7,132.8 232.9,123.2 269.1,117.9 305.3,102.9 341.5,97.2 377.6,80.1 413.8,75.0 450.0,80.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,173.3 124.4,153.0 160.5,147.3 196.7,132.8 232.9,145.5 269.1,109.6 305.3,98.0 341.5,85.2 377.6,83.5 413.8,90.1 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.4 124.4,167.8 160.5,147.3 196.7,136.3 232.9,140.9 269.1,109.5 305.3,101.3 341.5,83.1 377.6,77.7 413.8,81.6 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.3 88.2,176.3 124.4,153.0 160.5,148.3 196.7,136.3 232.9,126.7 269.1,109.5 305.3,101.4 341.5,86.6 377.6,90.9 413.8,80.4 450.0,76.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 0.818 ns | 0.938 ns | 0.943 ns | 0.818 ns |
| D38 | 1.61 ns | 1.4 ns | 1.81 ns | 1.6 ns | 1.6 ns |
| D57 | 2.51 ns | 1.63 ns | 2.26 ns | 1.88 ns | 2.25 ns |
| D76 | 2.08 ns | 2.68 ns | 3.1 ns | 3.08 ns | 3.09 ns |
| D115 | 4.86 ns | 5.55 ns | 5.56 ns | 4.85 ns | 4.85 ns |
| D153 | 5.28 ns | 8.45 ns | 4.72 ns | 5.13 ns | 7.62 ns |
| D230 | 17.7 ns | 13.7 ns | 16.1 ns | 16.1 ns | 16.2 ns |
| D307 | 23.5 ns | 23.5 ns | 25.1 ns | 23.6 ns | 23.5 ns |
| D462 | 33.6 ns | 31.6 ns | 47.1 ns | 37.3 ns | 37.3 ns |
| D616 | 34.2 ns | 52.3 ns | 45.6 ns | 49.7 ns | 35.5 ns |
| D924 | 84.7 ns | 75.5 ns | 56.8 ns | 76.1 ns | 84.9 ns |
| D1232 | 62.2 ns | 86.2 ns | 95.7 ns | 95.6 ns | 83.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,145.3 88.2,129.6 124.4,116.7 160.5,122.1 196.7,97.6 232.9,95.1 269.1,60.2 305.3,52.0 341.5,41.6 377.6,41.1 413.8,14.8 450.0,23.7 450.0,15.2 413.8,14.7 377.6,40.0 341.5,38.5 305.3,51.9 269.1,62.8 232.9,84.5 196.7,97.6 160.5,110.6 124.4,119.8 88.2,129.7 52.0,149.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,145.3 88.2,129.6 124.4,116.7 160.5,122.1 196.7,97.6 232.9,95.1 269.1,60.2 305.3,52.0 341.5,41.6 377.6,41.1 413.8,14.8 450.0,23.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,149.1 88.2,133.6 124.4,129.2 160.5,114.8 196.7,93.7 232.9,81.6 269.1,67.6 305.3,51.9 341.5,43.4 377.6,28.8 413.8,18.1 450.0,14.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.2 88.2,126.1 124.4,119.7 160.5,110.6 196.7,93.7 232.9,98.4 269.1,62.8 305.3,50.0 341.5,31.8 377.6,32.7 413.8,26.4 450.0,11.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.0 88.2,129.7 124.4,125.1 160.5,110.8 196.7,97.6 232.9,96.0 269.1,62.8 305.3,51.8 341.5,38.5 377.6,30.3 413.8,17.9 450.0,11.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,149.1 88.2,129.7 124.4,119.8 160.5,110.6 196.7,97.6 232.9,84.5 269.1,62.8 305.3,51.9 341.5,38.5 377.6,40.0 413.8,14.7 450.0,15.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
