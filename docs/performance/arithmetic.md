# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.939 ns | 1.06 ns | 0.937 ns | 0.935 ns | 0.941 ns |
| D38 | 1.62 ns | 1.62 ns | 1.61 ns | 1.41 ns | 1.07 ns |
| D57 | 2.25 ns | 2.25 ns | 2.25 ns | 2.25 ns | 1.94 ns |
| D76 | 3.48 ns | 1.82 ns | 3.11 ns | 3.08 ns | 3.09 ns |
| D115 | 4.99 ns | 4.39 ns | 4.4 ns | 4.4 ns | 4.4 ns |
| D153 | 5.9 ns | 5.89 ns | 5.89 ns | 5.89 ns | 5.89 ns |
| D230 | 13.9 ns | 15.3 ns | 10.1 ns | 8 ns | 13.4 ns |
| D307 | 18.5 ns | 14.6 ns | 19.6 ns | 15.8 ns | 19.6 ns |
| D462 | 32.8 ns | 38.9 ns | 29.5 ns | 29.5 ns | 32.8 ns |
| D616 | 45.6 ns | 48.7 ns | 46.5 ns | 39.5 ns | 54.9 ns |
| D924 | 74.8 ns | 74.8 ns | 57.7 ns | 61.7 ns | 74.6 ns |
| D1232 | 79 ns | 76.8 ns | 103 ns | 86 ns | 76 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.5 124.4,142.4 160.5,132.9 196.7,125.1 232.9,121.5 269.1,102.9 305.3,96.6 341.5,84.2 377.6,77.0 413.8,66.3 450.0,65.1 450.0,65.9 413.8,66.4 377.6,73.0 341.5,84.2 305.3,95.4 269.1,103.6 232.9,121.5 196.7,127.8 160.5,135.5 124.4,145.6 88.2,158.5 52.0,161.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.5 124.4,142.4 160.5,132.9 196.7,125.1 232.9,121.5 269.1,102.9 305.3,96.6 341.5,84.2 377.6,77.0 413.8,66.3 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.4 160.5,147.0 196.7,127.9 232.9,121.5 269.1,100.8 305.3,101.8 341.5,80.5 377.6,75.6 413.8,66.3 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.4 160.5,135.4 196.7,127.8 232.9,121.5 269.1,109.7 305.3,95.4 341.5,86.5 377.6,76.6 413.8,71.9 450.0,59.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,152.5 124.4,142.4 160.5,135.5 196.7,127.8 232.9,121.5 269.1,114.8 305.3,100.0 341.5,86.5 377.6,80.2 413.8,70.5 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,158.5 124.4,145.6 160.5,135.5 196.7,127.8 232.9,121.5 269.1,103.6 305.3,95.4 341.5,84.2 377.6,73.0 413.8,66.4 450.0,65.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.08 ns | 6.2 ns | 5.99 ns | 7.65 ns | 7.83 ns |
| D38 | 10 ns | 11.4 ns | 15.2 ns | 53.6 ns | 48.9 ns |
| D57 | 24 ns | 33.2 ns | 68.1 ns | 110 ns | 100 ns |
| D76 | 25.3 ns | 38.3 ns | 75.7 ns | 113 ns | 148 ns |
| D115 | 48.4 ns | 84 ns | 106 ns | 188 ns | 232 ns |
| D153 | 55.1 ns | 109 ns | 144 ns | 238 ns | 320 ns |
| D230 | 81.5 ns | 171 ns | 148 ns | 242 ns | 523 ns |
| D307 | 120 ns | 196 ns | 400 ns | 524 ns | 974 ns |
| D462 | 218 ns | 424 ns | 681 ns | 1.06 µs | 1.47 µs |
| D616 | 182 ns | 604 ns | 973 ns | 1.38 µs | 2.4 µs |
| D924 | 368 ns | 1.13 µs | 1.29 µs | 2.37 µs | 4.23 µs |
| D1232 | 360 ns | 1.28 µs | 3.56 µs | 3.65 µs | 7.64 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.8 88.2,160.0 124.4,141.0 160.5,139.8 196.7,125.8 232.9,122.9 269.1,114.4 305.3,106.1 341.5,93.1 377.6,97.0 413.8,81.7 450.0,82.2 450.0,15.8 413.8,28.7 377.6,41.0 341.5,51.6 305.3,60.6 269.1,74.1 232.9,84.7 196.7,91.8 160.5,101.5 124.4,109.9 88.2,125.5 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.8 88.2,160.0 124.4,141.0 160.5,139.8 196.7,125.8 232.9,122.9 269.1,114.4 305.3,106.1 341.5,93.1 377.6,97.0 413.8,81.7 450.0,82.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,157.1 124.4,133.9 160.5,130.9 196.7,113.8 232.9,108.1 269.1,98.4 305.3,95.3 341.5,78.6 377.6,70.9 413.8,57.4 450.0,54.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.1 88.2,151.0 124.4,118.3 160.5,116.1 196.7,108.6 232.9,102.2 269.1,101.5 305.3,79.9 341.5,68.3 377.6,60.6 413.8,54.5 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.8 88.2,123.5 124.4,107.9 160.5,107.3 196.7,96.3 232.9,91.1 269.1,90.8 305.3,74.0 341.5,58.8 377.6,53.0 413.8,41.2 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,125.5 124.4,109.9 160.5,101.5 196.7,91.8 232.9,84.7 269.1,74.1 305.3,60.6 341.5,51.6 377.6,41.0 413.8,28.7 450.0,15.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.882 ns | 3.06 ns | 3.11 ns | 4.92 ns | 5.12 ns |
| D38 | 3.5 ns | 14.4 ns | 26.7 ns | 29.8 ns | 15.4 ns |
| D57 | 6.23 ns | 21.3 ns | 32.8 ns | 72.7 ns | 59.6 ns |
| D76 | 5.64 ns | 25.4 ns | 42.1 ns | 78.7 ns | 99.8 ns |
| D115 | 13.5 ns | 51.1 ns | 84 ns | 196 ns | 229 ns |
| D153 | 18.1 ns | 52.5 ns | 112 ns | 235 ns | 357 ns |
| D230 | 27.9 ns | 132 ns | 247 ns | 288 ns | 811 ns |
| D307 | 44.4 ns | 146 ns | 509 ns | 854 ns | 1.46 µs |
| D462 | 99.6 ns | 408 ns | 1.27 µs | 1.78 µs | 2.61 µs |
| D616 | 67.4 ns | 672 ns | 1.44 µs | 2.05 µs | 4.19 µs |
| D924 | 145 ns | 1.48 µs | 2.01 µs | 4.46 µs | 7.55 µs |
| D1232 | 135 ns | 1.57 µs | 4.65 µs | 7.08 µs | 13 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.5 88.2,158.5 124.4,150.2 160.5,151.6 196.7,139.0 232.9,134.8 269.1,128.5 305.3,121.7 341.5,110.1 377.6,115.7 413.8,104.6 450.0,105.7 450.0,39.6 413.8,47.4 377.6,55.9 341.5,62.8 305.3,71.1 269.1,79.7 232.9,91.6 196.7,98.0 160.5,110.0 124.4,117.5 88.2,137.1 52.0,153.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.5 88.2,158.5 124.4,150.2 160.5,151.6 196.7,139.0 232.9,134.8 269.1,128.5 305.3,121.7 341.5,110.1 377.6,115.7 413.8,104.6 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.5 88.2,138.1 124.4,132.4 160.5,129.8 196.7,119.7 232.9,119.3 269.1,106.0 305.3,104.5 341.5,89.6 377.6,82.4 413.8,71.0 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,129.1 124.4,126.1 160.5,122.5 196.7,112.5 232.9,108.4 269.1,96.9 305.3,86.4 341.5,73.2 377.6,71.4 413.8,66.6 450.0,54.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,127.5 124.4,114.6 160.5,113.5 196.7,100.3 232.9,97.6 269.1,94.7 305.3,79.0 341.5,68.3 377.6,66.3 413.8,55.0 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.0 88.2,137.1 124.4,117.5 160.5,110.0 196.7,98.0 232.9,91.6 269.1,79.7 305.3,71.1 341.5,62.8 377.6,55.9 413.8,47.4 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.703 ns | 0.623 ns | 0.623 ns | 0.622 ns |
| D38 | 1.33 ns | 1.32 ns | 1.32 ns | 1.12 ns | 0.808 ns |
| D57 | 1.87 ns | 1.87 ns | 1.87 ns | 1.87 ns | 1.35 ns |
| D76 | 2.16 ns | 1.48 ns | 2.18 ns | 2.19 ns | 2.49 ns |
| D115 | 3.16 ns | 2.84 ns | 2.83 ns | 3.28 ns | 3.28 ns |
| D153 | 3.79 ns | 3.79 ns | 4.29 ns | 4.29 ns | 4.3 ns |
| D230 | 6 ns | 6.66 ns | 4.01 ns | 3.64 ns | 5.16 ns |
| D307 | 10.9 ns | 7.98 ns | 12.4 ns | 7.5 ns | 12.4 ns |
| D462 | 17.5 ns | 22.9 ns | 14.9 ns | 14.9 ns | 17.4 ns |
| D616 | 14.8 ns | 20.1 ns | 20.9 ns | 18.1 ns | 21.8 ns |
| D924 | 54.8 ns | 76.5 ns | 47.7 ns | 67.8 ns | 75.7 ns |
| D1232 | 38.4 ns | 52.8 ns | 68.8 ns | 51.8 ns | 44.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,135.1 124.4,125.2 160.5,121.0 196.7,110.0 232.9,104.8 269.1,91.5 305.3,74.2 341.5,60.5 377.6,65.2 413.8,27.4 450.0,37.7 450.0,33.7 413.8,18.1 377.6,54.1 341.5,60.6 305.3,70.6 269.1,95.8 232.9,101.1 196.7,108.9 160.5,116.9 124.4,134.7 88.2,149.5 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,135.1 124.4,125.2 160.5,121.0 196.7,110.0 232.9,104.8 269.1,91.5 305.3,74.2 341.5,60.5 377.6,65.2 413.8,27.4 450.0,37.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,132.0 196.7,113.2 232.9,104.8 269.1,88.5 305.3,83.2 341.5,52.7 377.6,56.4 413.8,17.7 450.0,28.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,120.7 196.7,113.2 232.9,101.1 269.1,103.1 305.3,70.5 341.5,65.0 377.6,55.3 413.8,31.5 450.0,20.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,140.0 124.4,125.2 160.5,120.7 196.7,108.9 232.9,101.2 269.1,105.9 305.3,85.0 341.5,65.0 377.6,59.5 413.8,21.3 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,149.5 124.4,134.7 160.5,116.9 196.7,108.9 232.9,101.1 269.1,95.8 305.3,70.6 341.5,60.6 377.6,54.1 413.8,18.1 450.0,33.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 1.87 ns | 2.18 ns | 2.18 ns |
| D38 | 3.25 ns | 3.24 ns | 3.25 ns | 2.74 ns | 1.6 ns |
| D57 | 7.17 ns | 7.16 ns | 7.17 ns | 7.16 ns | 6.27 ns |
| D76 | 9.52 ns | 5.5 ns | 8.43 ns | 8.43 ns | 8.59 ns |
| D115 | 14.4 ns | 12.7 ns | 12.7 ns | 12.8 ns | 12.7 ns |
| D153 | 16.5 ns | 17.1 ns | 17.6 ns | 17.7 ns | 17.8 ns |
| D230 | 32.1 ns | 36 ns | 16.8 ns | 17.7 ns | 22.7 ns |
| D307 | 41.9 ns | 30.6 ns | 48 ns | 31.9 ns | 47.8 ns |
| D462 | 90 ns | 80.3 ns | 72.8 ns | 72.7 ns | 84 ns |
| D616 | 71.3 ns | 84.5 ns | 72.4 ns | 63.3 ns | 99.9 ns |
| D924 | 101 ns | 102 ns | 70.9 ns | 62.3 ns | 80.4 ns |
| D1232 | 92.5 ns | 97 ns | 132 ns | 118 ns | 78.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,175.8 124.4,153.0 160.5,144.8 196.7,132.8 232.9,128.8 269.1,109.5 305.3,101.9 341.5,79.7 377.6,86.5 413.8,76.3 450.0,78.9 450.0,83.8 413.8,83.0 377.6,76.7 341.5,81.7 305.3,98.0 269.1,119.6 232.9,126.7 196.7,136.3 160.5,147.7 124.4,156.8 88.2,196.3 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,175.8 124.4,153.0 160.5,144.8 196.7,132.8 232.9,128.8 269.1,109.5 305.3,101.9 341.5,79.7 377.6,86.5 413.8,76.3 450.0,78.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,175.9 124.4,153.0 160.5,160.6 196.7,136.3 232.9,127.8 269.1,106.3 305.3,111.0 341.5,83.0 377.6,81.5 413.8,76.1 450.0,77.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,148.3 196.7,136.3 232.9,127.0 269.1,128.3 305.3,97.9 341.5,85.9 377.6,86.0 413.8,86.6 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,180.8 124.4,153.0 160.5,148.3 196.7,136.3 232.9,126.7 269.1,126.8 305.3,109.8 341.5,85.9 377.6,89.9 413.8,90.4 450.0,72.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,196.3 124.4,156.8 160.5,147.7 196.7,136.3 232.9,126.7 269.1,119.6 305.3,98.0 341.5,81.7 377.6,76.7 413.8,83.0 450.0,83.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.937 ns | 1.05 ns | 0.935 ns | 0.937 ns | 0.938 ns |
| D38 | 1.62 ns | 1.6 ns | 1.6 ns | 1.41 ns | 0.796 ns |
| D57 | 2.27 ns | 2.27 ns | 2.27 ns | 2.27 ns | 1.94 ns |
| D76 | 3.45 ns | 2.12 ns | 3.09 ns | 3.09 ns | 3.09 ns |
| D115 | 5.55 ns | 4.9 ns | 4.91 ns | 4.92 ns | 4.9 ns |
| D153 | 7.64 ns | 7.65 ns | 7.66 ns | 7.65 ns | 7.65 ns |
| D230 | 16.1 ns | 17.6 ns | 11.1 ns | 9.48 ns | 14.9 ns |
| D307 | 23.1 ns | 18.1 ns | 25.1 ns | 18.4 ns | 25.1 ns |
| D462 | 40.6 ns | 42.3 ns | 37.2 ns | 37.1 ns | 42.4 ns |
| D616 | 49.5 ns | 49.7 ns | 51 ns | 43.4 ns | 50.6 ns |
| D924 | 74.8 ns | 75 ns | 59.9 ns | 69.3 ns | 74.8 ns |
| D1232 | 87.6 ns | 85.3 ns | 104 ns | 86.9 ns | 84.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.6 124.4,142.2 160.5,133.1 196.7,122.8 232.9,115.8 269.1,99.7 305.3,91.9 341.5,79.6 377.6,75.3 413.8,66.3 450.0,62.9 450.0,63.8 413.8,66.3 377.6,74.8 341.5,78.6 305.3,90.0 269.1,101.3 232.9,115.8 196.7,125.5 160.5,135.5 124.4,145.6 88.2,165.0 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.2 160.5,133.1 196.7,122.8 232.9,115.8 269.1,99.7 305.3,91.9 341.5,79.6 377.6,75.3 413.8,66.3 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.7 124.4,142.2 160.5,143.6 196.7,125.5 232.9,115.8 269.1,97.7 305.3,97.2 341.5,78.7 377.6,75.2 413.8,66.2 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.7 124.4,142.2 160.5,135.5 196.7,125.5 232.9,115.8 269.1,107.8 305.3,90.0 341.5,81.5 377.6,74.6 413.8,71.1 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,152.5 124.4,142.2 160.5,135.5 196.7,125.4 232.9,115.8 269.1,111.2 305.3,96.7 341.5,81.5 377.6,78.1 413.8,68.0 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,165.0 124.4,145.6 160.5,135.5 196.7,125.5 232.9,115.8 269.1,101.3 305.3,90.0 341.5,78.6 377.6,74.8 413.8,66.3 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
