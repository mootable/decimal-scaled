# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 0.935 ns | 0.935 ns | 1.06 ns | 1.05 ns |
| D38 | 1.61 ns | 1.62 ns | 1.44 ns | 1.82 ns | 1.83 ns |
| D57 | 2.5 ns | 2.25 ns | 2.25 ns | 3.49 ns | 2.25 ns |
| D76 | 3.5 ns | 3.09 ns | 3.48 ns | 3.08 ns | 3.49 ns |
| D115 | 4.39 ns | 5.01 ns | 3.33 ns | 5 ns | 4.99 ns |
| D153 | 5.91 ns | 5.91 ns | 5.9 ns | 5.9 ns | 4.47 ns |
| D230 | 15.3 ns | 15.4 ns | 14 ns | 13.9 ns | 15.4 ns |
| D307 | 19.6 ns | 18.6 ns | 18.5 ns | 18.5 ns | 19.6 ns |
| D462 | 33.2 ns | 32.4 ns | 29.5 ns | 40.4 ns | 34.6 ns |
| D616 | 60.4 ns | 63.1 ns | 33.6 ns | 45.4 ns | 61 ns |
| D924 | 78.4 ns | 74.5 ns | 74.6 ns | 74.5 ns | 75.6 ns |
| D1232 | 66.4 ns | 110 ns | 92.5 ns | 94.8 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,149.6 124.4,140.1 160.5,132.8 196.7,127.9 232.9,121.4 269.1,100.7 305.3,95.4 341.5,83.9 377.6,71.0 413.8,65.3 450.0,68.9 450.0,58.6 413.8,66.1 377.6,70.7 341.5,83.0 305.3,95.4 269.1,100.7 232.9,127.5 196.7,125.1 160.5,132.9 124.4,142.4 88.2,146.9 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,149.6 124.4,140.1 160.5,132.8 196.7,127.9 232.9,121.4 269.1,100.7 305.3,95.4 341.5,83.9 377.6,71.0 413.8,65.3 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.4 160.5,135.5 196.7,125.0 232.9,121.4 269.1,100.6 305.3,96.5 341.5,84.4 377.6,70.0 413.8,66.4 450.0,58.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,152.1 124.4,142.4 160.5,132.9 196.7,133.9 232.9,121.5 269.1,102.7 305.3,96.6 341.5,86.5 377.6,83.7 413.8,66.4 450.0,61.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,147.0 124.4,132.9 160.5,135.6 196.7,125.1 232.9,121.5 269.1,102.8 305.3,96.6 341.5,79.7 377.6,77.2 413.8,66.4 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.4 160.5,132.9 196.7,125.1 232.9,127.5 269.1,100.7 305.3,95.4 341.5,83.0 377.6,70.7 413.8,66.1 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.68 ns | 5.94 ns | 5.61 ns | 9.91 ns | 9.93 ns |
| D38 | 9.66 ns | 11.2 ns | 14.6 ns | 811 ns | 1.07 µs |
| D57 | 35.6 ns | 49.1 ns | 66.9 ns | 110 ns | 101 ns |
| D76 | 41 ns | 60.9 ns | 85.2 ns | 104 ns | 133 ns |
| D115 | 54.6 ns | 84.7 ns | 90 ns | 187 ns | 231 ns |
| D153 | 66.4 ns | 109 ns | 143 ns | 218 ns | 300 ns |
| D230 | 110 ns | 168 ns | 230 ns | 367 ns | 586 ns |
| D307 | 145 ns | 220 ns | 357 ns | 562 ns | 928 ns |
| D462 | 229 ns | 435 ns | 648 ns | 1.12 µs | 1.32 µs |
| D616 | 270 ns | 660 ns | 954 ns | 1.78 µs | 2.34 µs |
| D924 | 352 ns | 1.06 µs | 2.02 µs | 2.52 µs | 4.25 µs |
| D1232 | 274 ns | 1.88 µs | 2.93 µs | 4.28 µs | 7.81 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,168.7 88.2,160.8 124.4,132.4 160.5,129.4 196.7,123.1 232.9,118.9 269.1,108.0 305.3,101.9 341.5,92.0 377.6,88.5 413.8,82.7 450.0,88.1 450.0,15.4 413.8,28.6 377.6,41.6 341.5,53.9 305.3,61.6 269.1,71.6 232.9,86.2 196.7,91.9 160.5,103.9 124.4,109.7 88.2,58.6 52.0,160.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,168.7 88.2,160.8 124.4,132.4 160.5,129.4 196.7,123.1 232.9,118.9 269.1,108.0 305.3,101.9 341.5,92.0 377.6,88.5 413.8,82.7 450.0,88.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.3 88.2,157.5 124.4,125.4 160.5,120.8 196.7,113.6 232.9,108.1 269.1,98.8 305.3,92.9 341.5,78.1 377.6,69.0 413.8,58.8 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.5 88.2,151.8 124.4,118.7 160.5,113.5 196.7,112.3 232.9,102.2 269.1,92.0 305.3,82.4 341.5,69.4 377.6,61.0 413.8,44.7 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,64.5 124.4,107.9 160.5,109.2 196.7,96.4 232.9,93.1 269.1,81.8 305.3,72.5 341.5,57.6 377.6,47.5 413.8,39.9 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.1 88.2,58.6 124.4,109.7 160.5,103.9 196.7,91.9 232.9,86.2 269.1,71.6 305.3,61.6 341.5,53.9 377.6,41.6 413.8,28.6 450.0,15.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3 ns | 3.11 ns | 5.02 ns | 4.97 ns |
| D38 | 3.5 ns | 14 ns | 25.7 ns | 50.4 ns | 30.3 ns |
| D57 | 4.22 ns | 21.2 ns | 32.3 ns | 77.7 ns | 72.2 ns |
| D76 | 5.71 ns | 34.2 ns | 44.7 ns | 79.5 ns | 107 ns |
| D115 | 13.1 ns | 57.3 ns | 76.2 ns | 214 ns | 252 ns |
| D153 | 18.7 ns | 54.2 ns | 112 ns | 234 ns | 337 ns |
| D230 | 27.8 ns | 126 ns | 337 ns | 518 ns | 1.06 µs |
| D307 | 55.1 ns | 167 ns | 459 ns | 1.03 µs | 1.45 µs |
| D462 | 86.8 ns | 451 ns | 1.27 µs | 1.87 µs | 2.4 µs |
| D616 | 103 ns | 727 ns | 1.65 µs | 2.66 µs | 4.17 µs |
| D924 | 140 ns | 1.48 µs | 2.93 µs | 4.86 µs | 7.33 µs |
| D1232 | 105 ns | 2.39 µs | 3.94 µs | 8.17 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,188.2 124.4,185.0 160.5,179.7 196.7,165.3 232.9,159.1 269.1,152.3 305.3,140.3 341.5,132.5 377.6,129.5 413.8,124.2 450.0,129.1 450.0,44.0 413.8,55.4 377.6,65.2 341.5,74.8 305.3,83.5 269.1,89.1 232.9,108.9 196.7,114.0 160.5,128.8 124.4,135.6 88.2,150.8 52.0,182.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,188.2 124.4,185.0 160.5,179.7 196.7,165.3 232.9,159.1 269.1,152.3 305.3,140.3 341.5,132.5 377.6,129.5 413.8,124.2 450.0,129.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.9 88.2,164.1 124.4,156.9 160.5,148.7 196.7,139.7 232.9,140.6 269.1,126.0 305.3,121.1 341.5,103.8 377.6,95.5 413.8,83.2 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.3 88.2,153.6 124.4,149.6 160.5,144.0 196.7,134.7 232.9,128.1 269.1,108.9 305.3,103.5 341.5,85.9 377.6,81.3 413.8,71.3 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,141.9 124.4,134.4 160.5,134.0 196.7,116.8 232.9,115.2 269.1,101.4 305.3,89.5 341.5,79.1 377.6,73.0 413.8,62.5 450.0,53.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.1 88.2,150.8 124.4,135.6 160.5,128.8 196.7,114.0 232.9,108.9 269.1,89.1 305.3,83.5 341.5,74.8 377.6,65.2 413.8,55.4 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.622 ns | 0.622 ns | 0.703 ns | 0.703 ns |
| D38 | 1.33 ns | 1.33 ns | 1.15 ns | 1.45 ns | 1.45 ns |
| D57 | 1.74 ns | 1.68 ns | 1.68 ns | 1.74 ns | 1.68 ns |
| D76 | 2.17 ns | 2.09 ns | 2.17 ns | 2.09 ns | 2.63 ns |
| D115 | 2.87 ns | 3.17 ns | 2.31 ns | 3.55 ns | 3.56 ns |
| D153 | 3.82 ns | 3.82 ns | 4.29 ns | 4.3 ns | 3.47 ns |
| D230 | 6.66 ns | 6.65 ns | 7.18 ns | 7.18 ns | 7.25 ns |
| D307 | 12.3 ns | 11.1 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 17.5 ns | 16.7 ns | 15.3 ns | 16.7 ns | 14.9 ns |
| D616 | 23 ns | 33.5 ns | 15.1 ns | 20 ns | 21.8 ns |
| D924 | 55.2 ns | 76.8 ns | 75.4 ns | 76.1 ns | 75.6 ns |
| D1232 | 21.4 ns | 70.4 ns | 64.2 ns | 61.4 ns | 69.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,120.9 196.7,112.9 232.9,104.6 269.1,88.4 305.3,70.8 341.5,60.4 377.6,52.5 413.8,27.2 450.0,54.7 450.0,20.4 413.8,18.1 377.6,54.1 341.5,65.1 305.3,70.2 269.1,86.0 232.9,107.3 196.7,106.6 160.5,115.3 124.4,128.3 88.2,132.6 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,120.9 196.7,112.9 232.9,104.6 269.1,88.4 305.3,70.8 341.5,60.4 377.6,52.5 413.8,27.2 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,122.0 196.7,110.0 232.9,104.5 269.1,88.5 305.3,73.7 341.5,61.9 377.6,41.7 413.8,17.6 450.0,20.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,139.2 124.4,128.3 160.5,120.9 196.7,119.1 232.9,101.2 269.1,86.2 305.3,73.7 341.5,64.3 377.6,64.8 413.8,18.2 450.0,22.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,127.3 160.5,122.0 196.7,106.6 232.9,101.1 269.1,86.3 305.3,73.7 341.5,61.9 377.6,56.6 413.8,17.9 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,128.3 160.5,115.3 196.7,106.6 232.9,107.3 269.1,86.0 305.3,70.2 341.5,65.1 377.6,54.1 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.87 ns | 1.87 ns | 2.46 ns | 2.46 ns |
| D38 | 6.96 ns | 6.98 ns | 11 ns | 19 ns | 16.3 ns |
| D57 | 8.09 ns | 7.16 ns | 7.17 ns | 8.1 ns | 7.17 ns |
| D76 | 9.53 ns | 8.72 ns | 9.52 ns | 8.43 ns | 9.84 ns |
| D115 | 12.8 ns | 14.1 ns | 9.26 ns | 14.1 ns | 14.1 ns |
| D153 | 17.1 ns | 15.8 ns | 15.9 ns | 16.2 ns | 12 ns |
| D230 | 36.3 ns | 36.4 ns | 32.1 ns | 31.9 ns | 36.3 ns |
| D307 | 48.5 ns | 40.3 ns | 43.6 ns | 43.1 ns | 47.8 ns |
| D462 | 89.2 ns | 87.1 ns | 74.4 ns | 90.3 ns | 77.7 ns |
| D616 | 101 ns | 123 ns | 62.2 ns | 78 ns | 94.5 ns |
| D924 | 113 ns | 102 ns | 100 ns | 93.9 ns | 92.1 ns |
| D1232 | 78.2 ns | 137 ns | 113 ns | 120 ns | 123 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,153.8 124.4,149.5 160.5,144.7 196.7,136.2 232.9,127.7 269.1,106.0 305.3,97.6 341.5,80.0 377.6,76.4 413.8,73.1 450.0,83.8 450.0,70.6 413.8,79.1 377.6,78.3 341.5,84.0 305.3,98.0 269.1,106.0 232.9,138.0 196.7,133.5 160.5,143.8 124.4,153.0 88.2,129.1 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,153.8 124.4,149.5 160.5,144.7 196.7,136.2 232.9,127.7 269.1,106.0 305.3,97.6 341.5,80.0 377.6,76.4 413.8,73.1 450.0,83.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,153.8 124.4,153.0 160.5,147.3 196.7,133.5 232.9,130.0 269.1,105.9 305.3,103.0 341.5,80.7 377.6,70.7 413.8,76.1 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,140.7 124.4,153.0 160.5,144.8 196.7,145.6 232.9,129.9 269.1,109.6 305.3,100.7 341.5,85.2 377.6,90.4 413.8,76.6 450.0,73.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,124.7 124.4,149.4 160.5,148.3 196.7,133.5 232.9,129.3 269.1,109.8 305.3,101.0 341.5,79.6 377.6,83.8 413.8,78.5 450.0,71.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,129.1 124.4,153.0 160.5,143.8 196.7,133.5 232.9,138.0 269.1,106.0 305.3,98.0 341.5,84.0 377.6,78.3 413.8,79.1 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.934 ns | 0.934 ns | 1.05 ns | 1.05 ns |
| D38 | 1.61 ns | 1.61 ns | 1.44 ns | 1.81 ns | 1.82 ns |
| D57 | 2.5 ns | 2.25 ns | 2.26 ns | 3.77 ns | 2.25 ns |
| D76 | 3.45 ns | 3.08 ns | 3.46 ns | 3.08 ns | 3.45 ns |
| D115 | 4.82 ns | 5.56 ns | 4.03 ns | 5.55 ns | 5.56 ns |
| D153 | 7.62 ns | 7.67 ns | 7.63 ns | 7.63 ns | 6.38 ns |
| D230 | 17.6 ns | 17.7 ns | 16.1 ns | 16.1 ns | 17.7 ns |
| D307 | 25.1 ns | 23.4 ns | 23.4 ns | 23.4 ns | 25.1 ns |
| D462 | 43.4 ns | 40.5 ns | 37.7 ns | 48.3 ns | 41.5 ns |
| D616 | 58.3 ns | 65 ns | 41 ns | 45 ns | 62.2 ns |
| D924 | 78.5 ns | 74.8 ns | 74.6 ns | 74.8 ns | 77 ns |
| D1232 | 65.4 ns | 107 ns | 92.7 ns | 95.5 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,149.6 124.4,140.1 160.5,133.1 196.7,125.8 232.9,115.9 269.1,97.7 305.3,90.0 341.5,78.1 377.6,71.7 413.8,65.3 450.0,69.2 450.0,58.7 413.8,65.7 377.6,70.3 341.5,79.1 305.3,90.0 269.1,97.6 232.9,119.8 196.7,122.7 160.5,133.1 124.4,142.4 88.2,147.0 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,149.6 124.4,140.1 160.5,133.1 196.7,125.8 232.9,115.9 269.1,97.7 305.3,90.0 341.5,78.1 377.6,71.7 413.8,65.3 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.4 160.5,135.6 196.7,122.7 232.9,115.8 269.1,97.6 305.3,91.5 341.5,79.6 377.6,69.4 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,152.0 124.4,142.3 160.5,133.1 196.7,129.8 232.9,115.9 269.1,99.7 305.3,91.5 341.5,81.2 377.6,79.4 413.8,66.3 450.0,61.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,131.2 160.5,135.6 196.7,122.8 232.9,115.9 269.1,99.6 305.3,91.5 341.5,75.8 377.6,77.4 413.8,66.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.0 124.4,142.4 160.5,133.1 196.7,122.7 232.9,119.8 269.1,97.6 305.3,90.0 341.5,79.1 377.6,70.3 413.8,65.7 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
