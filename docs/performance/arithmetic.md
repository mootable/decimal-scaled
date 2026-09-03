# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.944 ns | 1.06 ns | 1.06 ns | 0.935 ns | 0.939 ns |
| D38 | 1.61 ns | 0.941 ns | 0.922 ns | 1.64 ns | 1.14 ns |
| D57 | 2.25 ns | 1.21 ns | 1.29 ns | 1.24 ns | 1.39 ns |
| D76 | 3.08 ns | 1.6 ns | 3.08 ns | 2.86 ns | 1.84 ns |
| D115 | 5 ns | 4.39 ns | 4.99 ns | 4.39 ns | 4.42 ns |
| D153 | 6.65 ns | 5.89 ns | 5.92 ns | 5.89 ns | 6.63 ns |
| D230 | 13.9 ns | 13.9 ns | 15.4 ns | 15.4 ns | 13.9 ns |
| D307 | 19.6 ns | 11.8 ns | 11.8 ns | 18.5 ns | 18.5 ns |
| D462 | 32.6 ns | 29.6 ns | 29.6 ns | 33.8 ns | 28.9 ns |
| D616 | 34.8 ns | 49.9 ns | 59.7 ns | 51.1 ns | 39.3 ns |
| D924 | 74.5 ns | 84.9 ns | 75.8 ns | 74.8 ns | 85 ns |
| D1232 | 96.1 ns | 96.5 ns | 95.1 ns | 86.4 ns | 96.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,145.0 88.2,129.5 124.4,119.9 160.5,110.8 196.7,96.7 232.9,88.5 269.1,67.1 305.3,57.2 341.5,42.5 377.6,40.6 413.8,18.5 450.0,11.1 450.0,10.9 413.8,14.7 377.6,37.1 341.5,45.9 305.3,58.8 269.1,67.1 232.9,88.6 196.7,100.3 160.5,125.6 124.4,133.8 88.2,139.6 52.0,145.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,145.0 88.2,129.5 124.4,119.9 160.5,110.8 196.7,96.7 232.9,88.5 269.1,67.1 305.3,57.2 341.5,42.5 377.6,40.6 413.8,18.5 450.0,11.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,141.8 88.2,145.1 124.4,137.8 160.5,129.7 196.7,100.5 232.9,92.0 269.1,67.1 305.3,71.9 341.5,45.3 377.6,30.1 413.8,14.8 450.0,11.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.6 88.2,145.7 124.4,136.0 160.5,110.7 196.7,96.8 232.9,91.9 269.1,64.2 305.3,71.9 341.5,45.2 377.6,24.9 413.8,18.0 450.0,11.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.3 88.2,129.0 124.4,137.2 160.5,112.9 196.7,100.5 232.9,92.0 269.1,64.2 305.3,58.8 341.5,41.4 377.6,29.4 413.8,18.4 450.0,14.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.2 88.2,139.6 124.4,133.8 160.5,125.6 196.7,100.3 232.9,88.6 269.1,67.1 305.3,58.8 341.5,45.9 377.6,37.1 413.8,14.7 450.0,10.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.08 ns | 6.2 ns | 7.03 ns | 7.65 ns | 7.86 ns |
| D38 | 9.79 ns | 7.53 ns | 9.32 ns | 58.9 ns | 53.4 ns |
| D57 | 23.4 ns | 21.3 ns | 39.6 ns | 73.7 ns | 86.1 ns |
| D76 | 26.2 ns | 33.3 ns | 75.3 ns | 105 ns | 111 ns |
| D115 | 54 ns | 82.6 ns | 112 ns | 184 ns | 230 ns |
| D153 | 57.7 ns | 110 ns | 142 ns | 246 ns | 352 ns |
| D230 | 83.2 ns | 155 ns | 253 ns | 438 ns | 571 ns |
| D307 | 143 ns | 131 ns | 225 ns | 585 ns | 879 ns |
| D462 | 221 ns | 422 ns | 690 ns | 1.13 µs | 1.35 µs |
| D616 | 195 ns | 606 ns | 1.14 µs | 1.99 µs | 2.32 µs |
| D924 | 352 ns | 1.21 µs | 2.1 µs | 2.48 µs | 4.68 µs |
| D1232 | 512 ns | 1.78 µs | 3.54 µs | 3.64 µs | 7.04 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.8 88.2,160.5 124.4,141.5 160.5,139.1 196.7,123.4 232.9,121.9 269.1,114.0 305.3,102.3 341.5,92.8 377.6,95.5 413.8,82.7 450.0,74.6 450.0,17.6 413.8,26.5 377.6,41.7 341.5,53.5 305.3,62.8 269.1,72.2 232.9,82.6 196.7,91.9 160.5,107.8 124.4,113.2 88.2,123.6 52.0,165.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.8 88.2,160.5 124.4,141.5 160.5,139.1 196.7,123.4 232.9,121.9 269.1,114.0 305.3,102.3 341.5,92.8 377.6,95.5 413.8,82.7 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,166.1 124.4,143.6 160.5,133.9 196.7,114.1 232.9,107.9 269.1,100.5 305.3,104.2 341.5,78.7 377.6,70.9 413.8,55.8 450.0,47.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.6 88.2,161.5 124.4,130.1 160.5,116.2 196.7,107.5 232.9,102.3 269.1,89.9 305.3,92.4 341.5,68.1 377.6,57.2 413.8,43.9 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.8 88.2,121.5 124.4,116.6 160.5,108.9 196.7,96.7 232.9,90.5 269.1,77.9 305.3,71.6 341.5,57.3 377.6,45.1 413.8,40.3 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.2 88.2,123.6 124.4,113.2 160.5,107.8 196.7,91.9 232.9,82.6 269.1,72.2 305.3,62.8 341.5,53.5 377.6,41.7 413.8,26.5 450.0,17.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.882 ns | 3.07 ns | 3.2 ns | 4.93 ns | 5.05 ns |
| D38 | 3.5 ns | 9.43 ns | 15.4 ns | 25.1 ns | 20.9 ns |
| D57 | 6.23 ns | 14.2 ns | 21.9 ns | 47.2 ns | 53.4 ns |
| D76 | 7.8 ns | 21 ns | 42.4 ns | 80 ns | 77.1 ns |
| D115 | 13.6 ns | 46.2 ns | 93.5 ns | 196 ns | 231 ns |
| D153 | 16.8 ns | 49.4 ns | 112 ns | 235 ns | 396 ns |
| D230 | 28.1 ns | 117 ns | 368 ns | 567 ns | 974 ns |
| D307 | 54.3 ns | 98.5 ns | 285 ns | 1.04 µs | 1.4 µs |
| D462 | 94.4 ns | 410 ns | 1.3 µs | 1.8 µs | 2.48 µs |
| D616 | 72.2 ns | 682 ns | 1.86 µs | 2.91 µs | 3.82 µs |
| D924 | 137 ns | 1.6 µs | 2.99 µs | 5 µs | 8.32 µs |
| D1232 | 186 ns | 2.23 µs | 4.7 µs | 7.02 µs | 13.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.5 88.2,158.5 124.4,150.2 160.5,146.9 196.7,138.9 232.9,135.8 269.1,128.4 305.3,118.8 341.5,110.8 377.6,114.7 413.8,105.5 450.0,101.0 450.0,39.4 413.8,46.0 377.6,57.3 341.5,63.5 305.3,71.8 269.1,77.1 232.9,90.1 196.7,97.9 160.5,113.8 124.4,119.1 88.2,132.7 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.5 88.2,158.5 124.4,150.2 160.5,146.9 196.7,138.9 232.9,135.8 269.1,128.4 305.3,118.8 341.5,110.8 377.6,114.7 413.8,105.5 450.0,101.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.4 88.2,144.2 124.4,138.3 160.5,132.6 196.7,121.2 232.9,120.2 269.1,107.8 305.3,110.2 341.5,89.6 377.6,82.2 413.8,69.8 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,137.1 124.4,132.0 160.5,122.4 196.7,111.0 232.9,108.4 269.1,91.1 305.3,94.8 341.5,72.9 377.6,67.7 413.8,60.8 450.0,54.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,130.0 124.4,120.9 160.5,113.2 196.7,100.3 232.9,97.6 269.1,84.9 305.3,76.0 341.5,68.1 377.6,61.2 413.8,53.4 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,132.7 124.4,119.1 160.5,113.8 196.7,97.9 232.9,90.1 269.1,77.1 305.3,71.8 341.5,63.5 377.6,57.3 413.8,46.0 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.703 ns | 0.704 ns | 0.623 ns | 0.623 ns |
| D38 | 1.32 ns | 0.822 ns | 0.822 ns | 1.32 ns | 1.08 ns |
| D57 | 1.87 ns | 1.05 ns | 1.08 ns | 1.07 ns | 1.26 ns |
| D76 | 2.09 ns | 1.39 ns | 2.1 ns | 1.93 ns | 1.79 ns |
| D115 | 3.17 ns | 2.85 ns | 3.17 ns | 3.29 ns | 3.29 ns |
| D153 | 4.22 ns | 3.82 ns | 4.29 ns | 4.29 ns | 4.6 ns |
| D230 | 6 ns | 5.99 ns | 7.43 ns | 7.44 ns | 7.17 ns |
| D307 | 12.3 ns | 5.49 ns | 5.49 ns | 11 ns | 11 ns |
| D462 | 18.1 ns | 15.4 ns | 15.3 ns | 17 ns | 15 ns |
| D616 | 12.8 ns | 19.9 ns | 21.6 ns | 21.9 ns | 15 ns |
| D924 | 55 ns | 85.2 ns | 76.3 ns | 77 ns | 84.8 ns |
| D1232 | 47.2 ns | 61.4 ns | 61.5 ns | 52.1 ns | 61.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,122.0 196.7,109.9 232.9,101.6 269.1,91.4 305.3,70.7 341.5,59.5 377.6,69.5 413.8,27.3 450.0,31.7 450.0,24.1 413.8,14.8 377.6,64.9 341.5,64.9 305.3,73.9 269.1,86.3 232.9,99.1 196.7,108.8 160.5,126.4 124.4,136.5 88.2,141.0 52.0,157.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,122.0 196.7,109.9 232.9,101.6 269.1,91.4 305.3,70.7 341.5,59.5 377.6,69.5 413.8,27.3 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,149.0 124.4,142.0 160.5,133.9 196.7,113.0 232.9,104.5 269.1,91.5 305.3,94.0 341.5,64.2 377.6,56.7 413.8,14.7 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,149.0 124.4,141.1 160.5,121.9 196.7,109.9 232.9,101.1 269.1,85.3 305.3,94.0 341.5,64.3 377.6,54.4 413.8,17.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,135.2 124.4,141.3 160.5,124.3 196.7,108.9 232.9,101.1 269.1,85.2 305.3,73.8 341.5,61.4 377.6,53.9 413.8,17.6 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,141.0 124.4,136.5 160.5,126.4 196.7,108.8 232.9,99.1 269.1,86.3 305.3,73.9 341.5,64.9 377.6,64.9 413.8,14.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 2.11 ns | 2.18 ns | 2.18 ns |
| D38 | 3.43 ns | 1.84 ns | 1.86 ns | 4.07 ns | 2.22 ns |
| D57 | 7.17 ns | 3.74 ns | 3.61 ns | 3.54 ns | 3.7 ns |
| D76 | 8.44 ns | 4.94 ns | 8.61 ns | 8.04 ns | 5.15 ns |
| D115 | 14.4 ns | 12.7 ns | 14.4 ns | 12.7 ns | 12.7 ns |
| D153 | 20.1 ns | 15.9 ns | 16 ns | 16.1 ns | 20.1 ns |
| D230 | 32.4 ns | 31.9 ns | 36.3 ns | 36.3 ns | 32 ns |
| D307 | 64.1 ns | 23.5 ns | 23.5 ns | 42.7 ns | 42.5 ns |
| D462 | 89.6 ns | 73.7 ns | 76.1 ns | 84 ns | 77.3 ns |
| D616 | 66 ns | 81.9 ns | 97.1 ns | 96.4 ns | 62.2 ns |
| D924 | 108 ns | 117 ns | 103 ns | 99.9 ns | 109 ns |
| D1232 | 144 ns | 125 ns | 118 ns | 128 ns | 111 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,174.3 124.4,153.0 160.5,148.3 196.7,132.8 232.9,123.2 269.1,109.3 305.3,89.5 341.5,79.8 377.6,88.7 413.8,74.5 450.0,66.0 450.0,73.7 413.8,74.2 377.6,90.4 341.5,84.1 305.3,101.4 269.1,109.6 232.9,123.2 196.7,136.3 160.5,162.6 124.4,172.1 88.2,186.9 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,174.3 124.4,153.0 160.5,148.3 196.7,132.8 232.9,123.2 269.1,109.3 305.3,89.5 341.5,79.8 377.6,88.7 413.8,74.5 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,192.3 124.4,171.8 160.5,163.7 196.7,136.3 232.9,129.8 269.1,109.7 305.3,118.6 341.5,85.5 377.6,82.4 413.8,72.2 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,192.0 124.4,172.8 160.5,147.7 196.7,132.8 232.9,129.8 269.1,106.0 305.3,118.6 341.5,84.6 377.6,77.5 413.8,75.7 450.0,71.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,169.4 124.4,173.4 160.5,149.7 196.7,136.3 232.9,129.6 269.1,106.0 305.3,101.3 341.5,81.7 377.6,77.7 413.8,76.7 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,186.9 124.4,172.1 160.5,162.6 196.7,136.3 232.9,123.2 269.1,109.6 305.3,101.4 341.5,84.1 377.6,90.4 413.8,74.2 450.0,73.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 ns | 1.06 ns | 1.05 ns | 1.25 ns | 1.25 ns |
| D38 | 1.61 ns | 0.795 ns | 0.799 ns | 1.64 ns | 1.05 ns |
| D57 | 2.28 ns | 1.3 ns | 1.35 ns | 1.33 ns | 1.53 ns |
| D76 | 3.08 ns | 1.83 ns | 3.08 ns | 2.85 ns | 2.15 ns |
| D115 | 5.6 ns | 4.83 ns | 5.55 ns | 4.84 ns | 4.85 ns |
| D153 | 8.47 ns | 7.58 ns | 7.57 ns | 7.61 ns | 8.48 ns |
| D230 | 16.1 ns | 16.2 ns | 17.7 ns | 17.7 ns | 16.1 ns |
| D307 | 24.9 ns | 13.4 ns | 13.5 ns | 23.1 ns | 23.2 ns |
| D462 | 40.6 ns | 37.2 ns | 37.2 ns | 41.8 ns | 37 ns |
| D616 | 35.8 ns | 49.7 ns | 60.1 ns | 49 ns | 35.5 ns |
| D924 | 74.8 ns | 85 ns | 75.6 ns | 75.1 ns | 84.9 ns |
| D1232 | 97.9 ns | 98.5 ns | 95.3 ns | 84.3 ns | 97.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,137.0 88.2,129.5 124.4,119.5 160.5,110.7 196.7,93.5 232.9,81.5 269.1,62.8 305.3,50.3 341.5,36.1 377.6,39.7 413.8,18.4 450.0,10.6 450.0,10.9 413.8,14.7 377.6,40.0 341.5,38.8 305.3,52.3 269.1,62.8 232.9,81.4 196.7,97.6 160.5,121.1 124.4,131.0 88.2,142.0 52.0,137.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,137.0 88.2,129.5 124.4,119.5 160.5,110.7 196.7,93.5 232.9,81.5 269.1,62.8 305.3,50.3 341.5,36.1 377.6,39.7 413.8,18.4 450.0,10.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,141.8 88.2,150.0 124.4,135.8 160.5,125.9 196.7,97.7 232.9,84.7 269.1,62.8 305.3,68.1 341.5,38.6 377.6,30.2 413.8,14.7 450.0,10.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,149.8 124.4,134.5 160.5,110.7 196.7,93.7 232.9,84.7 269.1,60.2 305.3,68.0 341.5,38.6 377.6,24.7 413.8,18.1 450.0,11.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.9 88.2,129.1 124.4,135.0 160.5,113.0 196.7,97.7 232.9,84.6 269.1,60.2 305.3,52.4 341.5,35.3 377.6,30.6 413.8,18.3 450.0,15.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.0 88.2,142.0 124.4,131.0 160.5,121.1 196.7,97.6 232.9,81.4 269.1,62.8 305.3,52.3 341.5,38.8 377.6,40.0 413.8,14.7 450.0,10.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
