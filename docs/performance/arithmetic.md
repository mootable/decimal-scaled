# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.819 ns | 1.25 ns | 1.25 ns | 1.06 ns | 1.14 ns |
| D38 | 1.18 ns | 1.81 ns | 1.82 ns | 1.41 ns | 1.61 ns |
| D57 | 2.25 ns | 1.94 ns | 2.25 ns | 3.25 ns | 2.25 ns |
| D76 | 3.49 ns | 3.48 ns | 2.23 ns | 2.71 ns | 3.09 ns |
| D115 | 5 ns | 4.39 ns | 4.4 ns | 3.54 ns | 3.88 ns |
| D153 | 5.9 ns | 5.91 ns | 5.9 ns | 5.9 ns | 5.14 ns |
| D230 | 9.9 ns | 8.1 ns | 13.9 ns | 14 ns | 13.9 ns |
| D307 | 16.2 ns | 19.6 ns | 18.6 ns | 19.6 ns | 18.6 ns |
| D462 | 29.9 ns | 25.8 ns | 49.5 ns | 33.1 ns | 26.3 ns |
| D616 | 45.3 ns | 45.4 ns | 61 ns | 59.8 ns | 44 ns |
| D924 | 74.7 ns | 81.6 ns | 83.1 ns | 84.8 ns | 82.4 ns |
| D1232 | 106 ns | 106 ns | 107 ns | 105 ns | 96.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.3 88.2,156.4 124.4,142.4 160.5,132.9 196.7,125.1 232.9,121.5 269.1,110.2 305.3,99.6 341.5,86.2 377.6,77.2 413.8,66.3 450.0,58.8 450.0,60.7 413.8,64.2 377.6,77.8 341.5,89.0 305.3,96.5 269.1,102.8 232.9,124.4 196.7,130.6 160.5,135.5 124.4,142.4 88.2,149.6 52.0,157.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.3 88.2,156.4 124.4,142.4 160.5,132.9 196.7,125.1 232.9,121.5 269.1,110.2 305.3,99.6 341.5,86.2 377.6,77.2 413.8,66.3 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.2 88.2,147.1 124.4,145.6 160.5,132.9 196.7,127.9 232.9,121.4 269.1,114.6 305.3,95.4 341.5,89.4 377.6,77.2 413.8,64.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,147.0 124.4,142.4 160.5,142.6 196.7,127.8 232.9,121.5 269.1,102.8 305.3,96.5 341.5,75.3 377.6,70.7 413.8,64.0 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,134.4 160.5,138.3 196.7,132.6 232.9,121.5 269.1,102.7 305.3,95.4 341.5,84.0 377.6,71.2 413.8,63.6 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,149.6 124.4,142.4 160.5,135.5 196.7,130.6 232.9,124.4 269.1,102.8 305.3,96.5 341.5,89.0 377.6,77.8 413.8,64.2 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.01 ns | 5.44 ns | 6 ns | 8.83 ns | 8.39 ns |
| D38 | 8.35 ns | 12.7 ns | 14.3 ns | 58.4 ns | 60.7 ns |
| D57 | 23.4 ns | 25.8 ns | 68.2 ns | 106 ns | 107 ns |
| D76 | 24.6 ns | 67.1 ns | 62.2 ns | 97.4 ns | 133 ns |
| D115 | 45.4 ns | 82.9 ns | 105 ns | 162 ns | 185 ns |
| D153 | 55.3 ns | 109 ns | 144 ns | 228 ns | 261 ns |
| D230 | 43 ns | 86.6 ns | 193 ns | 384 ns | 565 ns |
| D307 | 85.4 ns | 246 ns | 357 ns | 638 ns | 874 ns |
| D462 | 146 ns | 266 ns | 736 ns | 1.14 µs | 1.01 µs |
| D616 | 227 ns | 586 ns | 1.1 µs | 1.59 µs | 2.03 µs |
| D924 | 355 ns | 1.12 µs | 1.79 µs | 2.86 µs | 4.52 µs |
| D1232 | 530 ns | 1.94 µs | 3.86 µs | 4.35 µs | 7.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.0 88.2,163.9 124.4,141.6 160.5,140.4 196.7,127.1 232.9,122.9 269.1,128.3 305.3,113.4 341.5,101.7 377.6,92.2 413.8,82.5 450.0,73.8 450.0,16.5 413.8,27.2 377.6,44.7 341.5,59.8 305.3,62.9 269.1,72.4 232.9,89.2 196.7,96.7 160.5,103.8 124.4,108.5 88.2,120.9 52.0,163.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.0 88.2,163.9 124.4,141.6 160.5,140.4 196.7,127.1 232.9,122.9 269.1,128.3 305.3,113.4 341.5,101.7 377.6,92.2 413.8,82.5 450.0,73.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.2 88.2,154.8 124.4,139.4 160.5,118.7 196.7,114.1 232.9,108.1 269.1,113.1 305.3,90.5 341.5,88.8 377.6,71.6 413.8,57.6 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.1 88.2,152.2 124.4,118.3 160.5,120.3 196.7,109.0 232.9,102.2 269.1,95.7 305.3,82.4 341.5,66.7 377.6,57.9 413.8,47.3 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.7 88.2,121.7 124.4,108.8 160.5,110.6 196.7,99.5 232.9,92.2 269.1,80.8 305.3,69.8 341.5,57.2 377.6,49.9 413.8,37.2 450.0,28.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.8 88.2,120.9 124.4,108.5 160.5,103.8 196.7,96.7 232.9,89.2 269.1,72.4 305.3,62.9 341.5,59.8 377.6,44.7 413.8,27.2 450.0,16.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.816 ns | 2.87 ns | 3.1 ns | 5 ns | 4.38 ns |
| D38 | 3.46 ns | 13.7 ns | 30.1 ns | 31.6 ns | 27.6 ns |
| D57 | 6.23 ns | 17.1 ns | 34.1 ns | 71.8 ns | 71.8 ns |
| D76 | 5.7 ns | 38.7 ns | 36.8 ns | 64.7 ns | 102 ns |
| D115 | 13.7 ns | 47.1 ns | 85.1 ns | 176 ns | 196 ns |
| D153 | 18.4 ns | 50.1 ns | 111 ns | 232 ns | 308 ns |
| D230 | 15.3 ns | 64.9 ns | 296 ns | 520 ns | 970 ns |
| D307 | 31.3 ns | 186 ns | 464 ns | 1.07 µs | 1.39 µs |
| D462 | 56.1 ns | 306 ns | 1.09 µs | 1.86 µs | 1.62 µs |
| D616 | 93 ns | 653 ns | 1.87 µs | 2.32 µs | 3.24 µs |
| D924 | 143 ns | 1.49 µs | 2.56 µs | 5.44 µs | 7.73 µs |
| D1232 | 201 ns | 2.38 µs | 5.14 µs | 8.28 µs | 13 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,158.7 124.4,150.2 160.5,151.5 196.7,138.8 232.9,134.5 269.1,137.2 305.3,126.8 341.5,118.4 377.6,111.0 413.8,104.8 450.0,99.9 450.0,39.5 413.8,47.1 377.6,59.7 341.5,69.7 305.3,72.0 269.1,77.1 232.9,93.7 196.7,100.2 160.5,109.7 124.4,114.8 88.2,128.6 52.0,155.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,158.7 124.4,150.2 160.5,151.5 196.7,138.8 232.9,134.5 269.1,137.2 305.3,126.8 341.5,118.4 377.6,111.0 413.8,104.8 450.0,99.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,138.8 124.4,135.6 160.5,123.8 196.7,120.9 232.9,120.0 269.1,116.3 305.3,101.1 341.5,93.8 377.6,82.8 413.8,70.8 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,127.4 124.4,125.6 160.5,124.5 196.7,112.3 232.9,108.6 269.1,94.3 305.3,87.8 341.5,75.4 377.6,67.6 413.8,63.1 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,126.7 124.4,114.8 160.5,116.3 196.7,101.8 232.9,97.8 269.1,86.1 305.3,75.6 341.5,67.7 377.6,64.5 413.8,52.1 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.3 88.2,128.6 124.4,114.8 160.5,109.7 196.7,100.2 232.9,93.7 269.1,77.1 305.3,72.0 341.5,69.7 377.6,59.7 413.8,47.1 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.545 ns | 0.622 ns | 0.622 ns | 0.703 ns | 0.369 ns |
| D38 | 1.07 ns | 1.45 ns | 1.45 ns | 1.12 ns | 1.42 ns |
| D57 | 1.68 ns | 1.35 ns | 1.68 ns | 1.69 ns | 1.68 ns |
| D76 | 2.17 ns | 2.17 ns | 1.8 ns | 1.68 ns | 2.49 ns |
| D115 | 3.17 ns | 2.83 ns | 2.83 ns | 2.82 ns | 2.75 ns |
| D153 | 3.79 ns | 3.79 ns | 4.3 ns | 4.3 ns | 3.57 ns |
| D230 | 3.81 ns | 2.97 ns | 5.53 ns | 7.18 ns | 7.17 ns |
| D307 | 6.93 ns | 12.5 ns | 11.1 ns | 12.5 ns | 11.1 ns |
| D462 | 12.6 ns | 12.3 ns | 32.4 ns | 17 ns | 12.3 ns |
| D616 | 18.7 ns | 20.1 ns | 21.8 ns | 17.3 ns | 20.2 ns |
| D924 | 54.8 ns | 76.5 ns | 84.6 ns | 84.7 ns | 82.6 ns |
| D1232 | 54.6 ns | 69.8 ns | 69.8 ns | 62.5 ns | 61.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.9 88.2,141.3 124.4,128.3 160.5,120.9 196.7,110.0 232.9,104.7 269.1,104.6 305.3,87.3 341.5,70.0 377.6,58.5 413.8,27.4 450.0,27.5 450.0,24.0 413.8,15.5 377.6,56.4 341.5,70.6 305.3,73.7 269.1,86.3 232.9,106.5 196.7,114.1 160.5,116.9 124.4,128.2 88.2,133.3 52.0,172.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.9 88.2,141.3 124.4,128.3 160.5,120.9 196.7,110.0 232.9,104.7 269.1,104.6 305.3,87.3 341.5,70.0 377.6,58.5 413.8,27.4 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,132.6 124.4,134.6 160.5,121.0 196.7,113.2 232.9,104.8 269.1,111.8 305.3,70.2 341.5,70.6 377.6,56.5 413.8,17.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,128.3 160.5,126.3 196.7,113.2 232.9,101.1 269.1,93.8 305.3,73.7 341.5,42.6 377.6,54.1 413.8,14.9 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,139.9 124.4,128.2 160.5,128.3 196.7,113.3 232.9,101.1 269.1,86.3 305.3,70.2 341.5,61.4 377.6,60.7 413.8,14.8 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,133.3 124.4,128.2 160.5,116.9 196.7,114.1 232.9,106.5 269.1,86.3 305.3,73.7 341.5,70.6 377.6,56.4 413.8,15.5 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 ns | 1.87 ns | 1.87 ns | 2.46 ns | 3.25 ns |
| D38 | 2.52 ns | 3.53 ns | 3.52 ns | 2.74 ns | 3.19 ns |
| D57 | 7.16 ns | 6.27 ns | 7.17 ns | 7.27 ns | 7.16 ns |
| D76 | 9.83 ns | 9.78 ns | 6.07 ns | 7.63 ns | 8.72 ns |
| D115 | 14.4 ns | 12.8 ns | 12.8 ns | 9.2 ns | 11.2 ns |
| D153 | 16.4 ns | 16.2 ns | 16 ns | 17 ns | 15.6 ns |
| D230 | 16.6 ns | 17.9 ns | 23.3 ns | 32.1 ns | 32.1 ns |
| D307 | 32.9 ns | 48.1 ns | 43.3 ns | 59 ns | 42.5 ns |
| D462 | 56 ns | 58.6 ns | 94 ns | 84.6 ns | 48.6 ns |
| D616 | 83.4 ns | 81.7 ns | 95.4 ns | 75.3 ns | 79.4 ns |
| D924 | 107 ns | 111 ns | 136 ns | 105 ns | 100 ns |
| D1232 | 143 ns | 154 ns | 132 ns | 123 ns | 111 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,183.2 124.4,153.0 160.5,143.8 196.7,132.8 232.9,128.9 269.1,128.6 305.3,108.9 341.5,93.4 377.6,81.9 413.8,74.7 450.0,66.3 450.0,73.8 413.8,76.6 377.6,83.3 341.5,97.6 305.3,101.4 269.1,109.5 232.9,130.5 196.7,140.1 160.5,147.3 124.4,153.0 88.2,176.4 52.0,175.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,183.2 124.4,153.0 160.5,143.8 196.7,132.8 232.9,128.9 269.1,128.6 305.3,108.9 341.5,93.4 377.6,81.9 413.8,74.7 450.0,66.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,173.5 124.4,156.8 160.5,144.0 196.7,136.3 232.9,129.5 269.1,126.4 305.3,97.9 341.5,92.1 377.6,82.5 413.8,73.6 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,173.6 124.4,153.0 160.5,157.8 196.7,136.2 232.9,129.7 269.1,118.8 305.3,100.9 341.5,78.5 377.6,78.0 413.8,67.9 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,180.9 124.4,152.6 160.5,151.2 196.7,145.7 232.9,128.0 269.1,109.5 305.3,91.9 341.5,81.5 377.6,84.9 413.8,75.1 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.9 88.2,176.4 124.4,153.0 160.5,147.3 196.7,140.1 232.9,130.5 269.1,109.5 305.3,101.4 341.5,97.6 377.6,83.3 413.8,76.6 450.0,73.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.82 ns | 0.935 ns | 0.935 ns | 1.06 ns | 1.14 ns |
| D38 | 1.05 ns | 1.81 ns | 1.81 ns | 1.41 ns | 1.61 ns |
| D57 | 2.26 ns | 1.94 ns | 2.25 ns | 3.27 ns | 2.25 ns |
| D76 | 3.46 ns | 3.46 ns | 2.6 ns | 2.68 ns | 3.11 ns |
| D115 | 5.54 ns | 4.89 ns | 4.89 ns | 4.45 ns | 4.31 ns |
| D153 | 7.67 ns | 7.65 ns | 7.62 ns | 7.64 ns | 6.58 ns |
| D230 | 11 ns | 9.54 ns | 15.5 ns | 16.2 ns | 16.2 ns |
| D307 | 18.9 ns | 25.1 ns | 23.4 ns | 25.1 ns | 23.4 ns |
| D462 | 35.7 ns | 30.8 ns | 53.2 ns | 42.2 ns | 31.3 ns |
| D616 | 45.1 ns | 44.9 ns | 63 ns | 59.5 ns | 48 ns |
| D924 | 74.8 ns | 81.6 ns | 82.1 ns | 84.8 ns | 83.6 ns |
| D1232 | 106 ns | 106 ns | 107 ns | 104 ns | 97.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.3 88.2,159.0 124.4,142.3 160.5,133.1 196.7,122.8 232.9,115.8 269.1,107.9 305.3,96.2 341.5,82.4 377.6,77.3 413.8,66.3 450.0,58.7 450.0,60.6 413.8,63.9 377.6,76.0 341.5,85.2 305.3,91.6 269.1,99.6 232.9,119.1 196.7,128.3 160.5,135.3 124.4,142.4 88.2,149.7 52.0,157.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.3 88.2,159.0 124.4,142.3 160.5,133.1 196.7,122.8 232.9,115.8 269.1,107.9 305.3,96.2 341.5,82.4 377.6,77.3 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,147.1 124.4,145.6 160.5,133.1 196.7,125.5 232.9,115.8 269.1,111.0 305.3,90.0 341.5,85.6 377.6,77.4 413.8,64.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.1 124.4,142.4 160.5,139.2 196.7,125.6 232.9,115.9 269.1,100.5 305.3,91.5 341.5,73.7 377.6,70.0 413.8,64.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,134.3 160.5,138.6 196.7,127.6 232.9,115.8 269.1,99.6 305.3,90.0 341.5,78.7 377.6,71.3 413.8,63.6 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,149.7 124.4,142.4 160.5,135.3 196.7,128.3 232.9,119.1 269.1,99.6 305.3,91.6 341.5,85.2 377.6,76.0 413.8,63.9 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
