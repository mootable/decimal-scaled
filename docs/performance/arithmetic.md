# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.989 ns | 0.64 ns | 0.935 ns | 0.941 ns | 1.06 ns |
| D38 | 1.61 ns | 1.61 ns | 0.963 ns | 0.993 ns | 1.41 ns |
| D57 | 2.5 ns | 1.94 ns | 2.25 ns | 2.25 ns | 2.25 ns |
| D76 | 3.08 ns | 3.08 ns | 3.48 ns | 3.08 ns | 1.85 ns |
| D115 | 4.41 ns | 4.42 ns | 4.99 ns | 4.41 ns | 2.94 ns |
| D153 | 5.91 ns | 5.9 ns | 4.42 ns | 5.91 ns | 5.9 ns |
| D230 | 15.4 ns | 13.9 ns | 13.9 ns | 13.8 ns | 11.8 ns |
| D307 | 18.6 ns | 15.1 ns | 18.6 ns | 18.6 ns | 19.6 ns |
| D462 | 33.3 ns | 28.7 ns | 30.6 ns | 50.2 ns | 32.5 ns |
| D616 | 61.7 ns | 45.4 ns | 61.4 ns | 48.6 ns | 45.2 ns |
| D924 | 74.5 ns | 56.2 ns | 88 ns | 71 ns | 74.6 ns |
| D1232 | 120 ns | 104 ns | 107 ns | 66.5 ns | 94.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.2 88.2,149.6 124.4,140.1 160.5,135.5 196.7,127.8 232.9,121.4 269.1,100.7 305.3,96.6 341.5,83.9 377.6,70.5 413.8,66.4 450.0,56.1 450.0,61.1 413.8,66.4 377.6,77.3 341.5,84.4 305.3,95.4 269.1,106.4 232.9,121.5 196.7,136.6 160.5,146.7 124.4,142.3 88.2,152.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.2 88.2,149.6 124.4,140.1 160.5,135.5 196.7,127.8 232.9,121.4 269.1,100.7 305.3,96.6 341.5,83.9 377.6,70.5 413.8,66.4 450.0,56.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.7 88.2,149.7 124.4,145.6 160.5,135.6 196.7,127.7 232.9,121.5 269.1,102.8 305.3,101.0 341.5,87.1 377.6,77.1 413.8,72.5 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,160.8 124.4,142.4 160.5,132.9 196.7,125.1 232.9,127.7 269.1,102.9 305.3,96.6 341.5,85.7 377.6,70.6 413.8,62.8 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,160.2 124.4,142.4 160.5,135.5 196.7,127.8 232.9,121.4 269.1,102.9 305.3,96.5 341.5,75.0 377.6,75.7 413.8,67.4 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,142.3 160.5,146.7 196.7,136.6 232.9,121.5 269.1,106.4 305.3,95.4 341.5,84.4 377.6,77.3 413.8,66.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.91 ns | 3.87 ns | 5.53 ns | 7.47 ns | 9.04 ns |
| D38 | 9.73 ns | 11.4 ns | 9.87 ns | 55.9 ns | 54.3 ns |
| D57 | 21.6 ns | 26.8 ns | 68.6 ns | 111 ns | 113 ns |
| D76 | 26 ns | 60.1 ns | 83.7 ns | 114 ns | 111 ns |
| D115 | 43.1 ns | 82.4 ns | 112 ns | 193 ns | 179 ns |
| D153 | 58.8 ns | 110 ns | 122 ns | 241 ns | 320 ns |
| D230 | 94.9 ns | 156 ns | 231 ns | 398 ns | 451 ns |
| D307 | 118 ns | 192 ns | 359 ns | 588 ns | 974 ns |
| D462 | 217 ns | 431 ns | 687 ns | 1.17 µs | 1.49 µs |
| D616 | 247 ns | 611 ns | 1.12 µs | 1.82 µs | 2.16 µs |
| D924 | 367 ns | 1e+03 ns | 2.29 µs | 2.53 µs | 4.11 µs |
| D1232 | 564 ns | 1.76 µs | 3.82 µs | 3.24 µs | 6.95 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.4 88.2,160.6 124.4,143.3 160.5,139.2 196.7,128.3 232.9,121.5 269.1,111.1 305.3,106.3 341.5,93.2 377.6,90.4 413.8,81.8 450.0,72.4 450.0,17.9 413.8,29.3 377.6,43.3 341.5,51.4 305.3,60.6 269.1,77.3 232.9,84.7 196.7,97.3 160.5,107.8 124.4,107.4 88.2,123.3 52.0,162.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.4 88.2,160.6 124.4,143.3 160.5,139.2 196.7,128.3 232.9,121.5 269.1,111.1 305.3,106.3 341.5,93.2 377.6,90.4 413.8,81.8 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.6 88.2,157.1 124.4,138.6 160.5,121.1 196.7,114.2 232.9,108.0 269.1,100.3 305.3,95.8 341.5,78.3 377.6,70.7 413.8,60.0 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.9 88.2,160.3 124.4,118.2 160.5,113.9 196.7,107.5 232.9,105.8 269.1,91.8 305.3,82.3 341.5,68.1 377.6,57.5 413.8,42.0 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,166.3 88.2,122.6 124.4,107.7 160.5,107.2 196.7,95.7 232.9,90.9 269.1,80.0 305.3,71.5 341.5,56.6 377.6,47.0 413.8,39.8 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.2 88.2,123.3 124.4,107.4 160.5,107.8 196.7,97.3 232.9,84.7 269.1,77.3 305.3,60.6 341.5,51.4 377.6,43.3 413.8,29.3 450.0,17.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.661 ns | 1.6 ns | 3.02 ns | 5.27 ns | 4.94 ns |
| D38 | 3.5 ns | 14.1 ns | 17.1 ns | 17.7 ns | 22.9 ns |
| D57 | 4.22 ns | 16.9 ns | 33.9 ns | 71.3 ns | 71.6 ns |
| D76 | 8.1 ns | 35.2 ns | 45.4 ns | 79.6 ns | 74.7 ns |
| D115 | 13.1 ns | 46 ns | 93.5 ns | 196 ns | 180 ns |
| D153 | 18.1 ns | 50.7 ns | 90.7 ns | 236 ns | 357 ns |
| D230 | 27.9 ns | 117 ns | 336 ns | 522 ns | 680 ns |
| D307 | 44 ns | 143 ns | 455 ns | 1.03 µs | 1.47 µs |
| D462 | 103 ns | 409 ns | 1.26 µs | 1.87 µs | 2.63 µs |
| D616 | 123 ns | 674 ns | 1.86 µs | 2.72 µs | 3.93 µs |
| D924 | 138 ns | 1.33 µs | 3.22 µs | 4.61 µs | 7.52 µs |
| D1232 | 194 ns | 2.22 µs | 5.11 µs | 5.71 µs | 13.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.7 88.2,158.5 124.4,155.8 160.5,146.4 196.7,139.4 232.9,134.8 269.1,128.5 305.3,121.9 341.5,109.5 377.6,107.0 413.8,105.3 450.0,100.4 450.0,39.4 413.8,47.5 377.6,56.8 341.5,62.6 305.3,71.1 269.1,82.2 232.9,91.6 196.7,101.5 160.5,114.2 124.4,114.8 88.2,131.3 52.0,153.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.7 88.2,158.5 124.4,155.8 160.5,146.4 196.7,139.4 232.9,134.8 269.1,128.5 305.3,121.9 341.5,109.5 377.6,107.0 413.8,105.3 450.0,100.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.9 88.2,138.3 124.4,135.7 160.5,125.1 196.7,121.3 232.9,119.8 269.1,107.8 305.3,104.8 341.5,89.6 377.6,82.4 413.8,72.6 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.7 88.2,135.6 124.4,125.7 160.5,121.4 196.7,111.0 232.9,111.4 269.1,92.5 305.3,88.1 341.5,73.3 377.6,67.7 413.8,59.7 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.6 88.2,135.1 124.4,114.9 160.5,113.3 196.7,100.3 232.9,97.6 269.1,86.1 305.3,76.2 341.5,67.6 377.6,62.2 413.8,54.6 450.0,51.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,131.3 124.4,114.8 160.5,114.2 196.7,101.5 232.9,91.6 269.1,82.2 305.3,71.1 341.5,62.6 377.6,56.8 413.8,47.5 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.329 ns | 0.258 ns | 0.622 ns | 0.622 ns | 0.703 ns |
| D38 | 1.32 ns | 1.32 ns | 0.814 ns | 0.92 ns | 1.12 ns |
| D57 | 1.74 ns | 1.35 ns | 1.69 ns | 1.68 ns | 1.68 ns |
| D76 | 2.19 ns | 2.19 ns | 2.17 ns | 2.19 ns | 1.77 ns |
| D115 | 2.85 ns | 2.85 ns | 3.17 ns | 3.29 ns | 2.28 ns |
| D153 | 3.82 ns | 3.82 ns | 3.27 ns | 4.3 ns | 4.3 ns |
| D230 | 6.65 ns | 5.86 ns | 7.16 ns | 7.16 ns | 4.42 ns |
| D307 | 10.9 ns | 9.59 ns | 11.1 ns | 11.1 ns | 12.4 ns |
| D462 | 17.5 ns | 14.9 ns | 14.9 ns | 16.7 ns | 16.7 ns |
| D616 | 23.7 ns | 19.9 ns | 21.8 ns | 20 ns | 20 ns |
| D924 | 55 ns | 59.6 ns | 84.7 ns | 65.3 ns | 75.6 ns |
| D1232 | 77 ns | 65.7 ns | 69.8 ns | 36.5 ns | 61.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.5 88.2,135.2 124.4,127.3 160.5,120.7 196.7,113.0 232.9,104.6 269.1,88.5 305.3,74.2 341.5,60.4 377.6,51.7 413.8,27.3 450.0,17.5 450.0,24.1 413.8,18.1 377.6,56.6 341.5,61.9 305.3,70.5 269.1,100.3 232.9,101.1 196.7,119.4 160.5,126.7 124.4,128.2 88.2,140.0 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.5 88.2,135.2 124.4,127.3 160.5,120.7 196.7,113.0 232.9,104.6 269.1,88.5 305.3,74.2 341.5,60.4 377.6,51.7 413.8,27.3 450.0,17.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.6 88.2,135.2 124.4,134.7 160.5,120.7 196.7,113.0 232.9,104.5 269.1,92.1 305.3,77.9 341.5,65.1 377.6,56.7 413.8,25.0 450.0,22.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,149.3 124.4,128.2 160.5,120.9 196.7,110.0 232.9,109.0 269.1,86.4 305.3,73.7 341.5,65.1 377.6,54.1 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,145.7 124.4,128.3 160.5,120.7 196.7,108.9 232.9,101.1 269.1,86.3 305.3,73.7 341.5,61.9 377.6,56.6 413.8,22.3 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,140.0 124.4,128.2 160.5,126.7 196.7,119.4 232.9,101.1 269.1,100.3 305.3,70.5 341.5,61.9 377.6,56.6 413.8,18.1 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.75 ns | 1.4 ns | 1.87 ns | 2.18 ns | 2.46 ns |
| D38 | 3.3 ns | 3.31 ns | 1.97 ns | 2.04 ns | 2.74 ns |
| D57 | 8.09 ns | 6.28 ns | 7.17 ns | 7.16 ns | 7.17 ns |
| D76 | 8.43 ns | 8.71 ns | 9.84 ns | 8.43 ns | 4.87 ns |
| D115 | 12.8 ns | 12.7 ns | 14.4 ns | 12.7 ns | 7.56 ns |
| D153 | 16.4 ns | 16.2 ns | 12.7 ns | 16.8 ns | 17.5 ns |
| D230 | 36.5 ns | 32.2 ns | 32.1 ns | 31.9 ns | 19.6 ns |
| D307 | 41.3 ns | 37.3 ns | 42.9 ns | 42.5 ns | 47.8 ns |
| D462 | 88.3 ns | 73 ns | 77.1 ns | 87.4 ns | 82.6 ns |
| D616 | 104 ns | 81.3 ns | 96.6 ns | 78.9 ns | 77.8 ns |
| D924 | 110 ns | 70.8 ns | 122 ns | 82.2 ns | 91.2 ns |
| D1232 | 158 ns | 137 ns | 135 ns | 63.9 ns | 109 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,175.4 124.4,149.5 160.5,148.3 196.7,136.3 232.9,129.0 269.1,105.8 305.3,102.3 341.5,80.3 377.6,75.6 413.8,73.8 450.0,63.4 450.0,74.1 413.8,79.3 377.6,83.9 341.5,82.2 305.3,98.0 269.1,123.9 232.9,127.2 196.7,151.4 160.5,164.2 124.4,153.0 88.2,180.8 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,175.4 124.4,149.5 160.5,148.3 196.7,136.3 232.9,129.0 269.1,105.8 305.3,102.3 341.5,80.3 377.6,75.6 413.8,73.8 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.3 88.2,175.3 124.4,156.8 160.5,147.3 196.7,136.3 232.9,129.4 269.1,109.4 305.3,105.2 341.5,85.8 377.6,82.7 413.8,86.7 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,190.4 124.4,153.0 160.5,143.8 196.7,132.8 232.9,136.5 269.1,109.5 305.3,101.2 341.5,84.2 377.6,77.7 413.8,70.9 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,189.4 124.4,153.0 160.5,148.3 196.7,136.3 232.9,128.3 269.1,109.8 305.3,101.4 341.5,80.6 377.6,83.5 413.8,82.3 450.0,89.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,180.8 124.4,153.0 160.5,164.2 196.7,151.4 232.9,127.2 269.1,123.9 305.3,98.0 341.5,82.2 377.6,83.9 413.8,79.3 450.0,74.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1 ns | 0.647 ns | 0.935 ns | 0.935 ns | 1.05 ns |
| D38 | 1.61 ns | 1.6 ns | 0.893 ns | 0.907 ns | 1.41 ns |
| D57 | 2.51 ns | 1.94 ns | 2.27 ns | 2.27 ns | 2.28 ns |
| D76 | 3.09 ns | 3.09 ns | 3.46 ns | 3.09 ns | 2.16 ns |
| D115 | 4.84 ns | 4.86 ns | 5.55 ns | 4.83 ns | 3.69 ns |
| D153 | 7.57 ns | 7.57 ns | 5.99 ns | 7.55 ns | 7.54 ns |
| D230 | 17.7 ns | 16.1 ns | 16.1 ns | 16.1 ns | 13 ns |
| D307 | 23.7 ns | 19.5 ns | 23.5 ns | 23.5 ns | 25.2 ns |
| D462 | 43.2 ns | 37.2 ns | 39.2 ns | 56.8 ns | 40.5 ns |
| D616 | 64.1 ns | 46.1 ns | 62.6 ns | 46.1 ns | 45.9 ns |
| D924 | 74.8 ns | 60.9 ns | 86 ns | 78.1 ns | 74.8 ns |
| D1232 | 121 ns | 104 ns | 107 ns | 73.8 ns | 95.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.0 88.2,149.7 124.4,140.0 160.5,135.5 196.7,125.8 232.9,116.0 269.1,97.7 305.3,91.3 341.5,78.2 377.6,69.7 413.8,66.3 450.0,55.9 450.0,61.1 413.8,66.3 377.6,76.9 341.5,79.6 305.3,90.0 269.1,104.4 232.9,116.1 196.7,131.7 160.5,143.3 124.4,142.1 88.2,152.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.0 88.2,149.7 124.4,140.0 160.5,135.5 196.7,125.8 232.9,116.0 269.1,97.7 305.3,91.3 341.5,78.2 377.6,69.7 413.8,66.3 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.5 88.2,149.7 124.4,145.6 160.5,135.5 196.7,125.7 232.9,116.0 269.1,99.6 305.3,95.5 341.5,81.4 377.6,76.8 413.8,70.8 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,162.5 124.4,142.2 160.5,133.1 196.7,122.8 232.9,121.1 269.1,99.6 305.3,91.5 341.5,80.3 377.6,70.2 413.8,63.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,162.1 124.4,142.2 160.5,135.5 196.7,125.8 232.9,116.1 269.1,99.6 305.3,91.5 341.5,72.3 377.6,76.8 413.8,65.4 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,142.1 160.5,143.3 196.7,131.7 232.9,116.1 269.1,104.4 305.3,90.0 341.5,79.6 377.6,76.9 413.8,66.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
