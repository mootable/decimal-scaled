# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.842 ns | 1.25 ns | 1.05 ns | 1.03 ns | 1.06 ns |
| D38 | 1.13 ns | 1.83 ns | 1.61 ns | 1.61 ns | 1.81 ns |
| D57 | 1.94 ns | 2.5 ns | 2.28 ns | 2.28 ns | 2.5 ns |
| D76 | 1.78 ns | 3.48 ns | 3.09 ns | 3.49 ns | 1.56 ns |
| D115 | 2.85 ns | 4.42 ns | 2.36 ns | 5 ns | 4.99 ns |
| D153 | 5.91 ns | 4.2 ns | 5.9 ns | 4.64 ns | 5.89 ns |
| D230 | 13.8 ns | 15.3 ns | 15.4 ns | 13.3 ns | 11.9 ns |
| D307 | 19.6 ns | 18.7 ns | 11.5 ns | 19.6 ns | 19.6 ns |
| D462 | 39.9 ns | 32.7 ns | 32.6 ns | 43.1 ns | 32.7 ns |
| D616 | 45 ns | 60.1 ns | 45.3 ns | 61.3 ns | 45.2 ns |
| D924 | 56.9 ns | 60.3 ns | 84.8 ns | 74.6 ns | 98.4 ns |
| D1232 | 95 ns | 94.8 ns | 106 ns | 83.9 ns | 70.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,163.7 88.2,157.4 124.4,145.7 160.5,147.5 196.7,137.3 232.9,121.4 269.1,102.9 305.3,95.4 341.5,79.9 377.6,77.3 413.8,72.2 450.0,61.1 450.0,67.5 413.8,60.4 377.6,77.3 341.5,84.3 305.3,95.4 269.1,106.2 232.9,121.5 196.7,125.1 160.5,150.4 124.4,140.1 88.2,147.1 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,163.7 88.2,157.4 124.4,145.7 160.5,147.5 196.7,137.3 232.9,121.4 269.1,102.9 305.3,95.4 341.5,79.9 377.6,77.3 413.8,72.2 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.2 88.2,146.9 124.4,140.1 160.5,132.9 196.7,127.8 232.9,128.9 269.1,100.7 305.3,96.4 341.5,84.3 377.6,71.0 413.8,71.0 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.1 160.5,135.5 196.7,141.4 232.9,121.5 269.1,100.6 305.3,106.9 341.5,84.4 377.6,77.2 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,149.7 124.4,142.1 160.5,132.9 196.7,125.1 232.9,126.7 269.1,103.8 305.3,95.4 341.5,78.3 377.6,70.6 413.8,66.4 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,140.1 160.5,150.4 196.7,125.1 232.9,121.5 269.1,106.2 305.3,95.4 341.5,84.3 377.6,77.3 413.8,60.4 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.23 ns | 5.42 ns | 6.2 ns | 8.94 ns | 8.9 ns |
| D38 | 8.21 ns | 12.7 ns | 15.6 ns | 58.2 ns | 68.3 ns |
| D57 | 19.3 ns | 34.5 ns | 67.8 ns | 111 ns | 120 ns |
| D76 | 19.8 ns | 66.8 ns | 75.7 ns | 120 ns | 91.2 ns |
| D115 | 27.8 ns | 83.6 ns | 61.3 ns | 200 ns | 254 ns |
| D153 | 55 ns | 80.3 ns | 143 ns | 216 ns | 320 ns |
| D230 | 82.4 ns | 171 ns | 252 ns | 347 ns | 487 ns |
| D307 | 133 ns | 222 ns | 223 ns | 644 ns | 973 ns |
| D462 | 206 ns | 477 ns | 751 ns | 1.18 µs | 1.49 µs |
| D616 | 243 ns | 646 ns | 1.03 µs | 1.99 µs | 2.15 µs |
| D924 | 222 ns | 792 ns | 2.29 µs | 2.53 µs | 4.68 µs |
| D1232 | 516 ns | 1.8 µs | 3.81 µs | 3.62 µs | 7.65 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.7 88.2,164.3 124.4,145.7 160.5,145.1 196.7,137.8 232.9,123.0 269.1,114.2 305.3,103.8 341.5,94.3 377.6,90.8 413.8,92.7 450.0,74.4 450.0,15.8 413.8,26.5 377.6,43.4 341.5,51.3 305.3,60.6 269.1,75.6 232.9,84.8 196.7,89.7 160.5,112.0 124.4,106.0 88.2,118.3 52.0,162.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.7 88.2,164.3 124.4,145.7 160.5,145.1 196.7,137.8 232.9,123.0 269.1,114.2 305.3,103.8 341.5,94.3 377.6,90.8 413.8,92.7 450.0,74.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.3 88.2,154.9 124.4,133.1 160.5,118.7 196.7,113.9 232.9,114.8 269.1,98.3 305.3,92.7 341.5,76.1 377.6,69.5 413.8,65.1 450.0,47.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,150.3 124.4,118.5 160.5,116.1 196.7,120.6 232.9,102.3 269.1,89.9 305.3,92.6 341.5,66.2 377.6,59.3 413.8,42.0 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.4 88.2,121.7 124.4,107.7 160.5,106.1 196.7,94.9 232.9,93.3 269.1,83.0 305.3,69.5 341.5,56.4 377.6,45.0 413.8,39.9 450.0,32.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.5 88.2,118.3 124.4,106.0 160.5,112.0 196.7,89.7 232.9,84.8 269.1,75.6 305.3,60.6 341.5,51.3 377.6,43.4 413.8,26.5 450.0,15.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.478 ns | 2.87 ns | 3.19 ns | 5.23 ns | 4.98 ns |
| D38 | 2.92 ns | 13.7 ns | 25.3 ns | 26.3 ns | 36.9 ns |
| D57 | 3.27 ns | 21.6 ns | 32.8 ns | 72.5 ns | 76.7 ns |
| D76 | 4.42 ns | 39.4 ns | 43.4 ns | 84 ns | 63.3 ns |
| D115 | 10.6 ns | 60.8 ns | 49.8 ns | 213 ns | 254 ns |
| D153 | 18.1 ns | 36.5 ns | 112 ns | 218 ns | 357 ns |
| D230 | 28.1 ns | 132 ns | 368 ns | 459 ns | 807 ns |
| D307 | 54.3 ns | 167 ns | 285 ns | 1.08 µs | 1.46 µs |
| D462 | 72.9 ns | 449 ns | 1.32 µs | 1.87 µs | 2.65 µs |
| D616 | 95.7 ns | 732 ns | 1.75 µs | 2.91 µs | 3.9 µs |
| D924 | 102 ns | 1.01 µs | 3.24 µs | 5.04 µs | 8.35 µs |
| D1232 | 183 ns | 2.2 µs | 5.1 µs | 7.05 µs | 13 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.4 88.2,161.2 124.4,159.5 160.5,155.2 196.7,142.4 232.9,134.8 269.1,128.4 305.3,118.8 341.5,114.6 377.6,110.6 413.8,109.8 450.0,101.2 450.0,39.5 413.8,45.9 377.6,57.0 341.5,62.6 305.3,71.2 269.1,79.8 232.9,91.6 196.7,96.5 160.5,116.6 124.4,113.8 88.2,124.4 52.0,153.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.4 88.2,161.2 124.4,159.5 160.5,155.2 196.7,142.4 232.9,134.8 269.1,128.4 305.3,118.8 341.5,114.6 377.6,110.6 413.8,109.8 450.0,101.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,138.7 124.4,132.2 160.5,123.5 196.7,117.2 232.9,124.6 269.1,106.0 305.3,102.6 341.5,88.3 377.6,81.2 413.8,76.6 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.9 88.2,129.9 124.4,126.1 160.5,122.1 196.7,120.1 232.9,108.4 269.1,91.1 305.3,94.8 341.5,72.7 377.6,68.6 413.8,59.7 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.7 88.2,129.3 124.4,114.7 160.5,112.5 196.7,99.0 232.9,98.7 269.1,87.9 305.3,75.5 341.5,67.6 377.6,61.2 413.8,53.3 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,124.4 124.4,113.8 160.5,116.6 196.7,96.5 232.9,91.6 269.1,79.8 305.3,71.2 341.5,62.6 377.6,57.0 413.8,45.9 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.271 ns | 0.622 ns | 0.703 ns | 0.346 ns | 0.703 ns |
| D38 | 1.03 ns | 1.45 ns | 1.33 ns | 1.33 ns | 1.45 ns |
| D57 | 1.35 ns | 1.74 ns | 1.68 ns | 1.68 ns | 1.74 ns |
| D76 | 1.52 ns | 2.16 ns | 2.1 ns | 2.17 ns | 1.53 ns |
| D115 | 2.18 ns | 2.85 ns | 1.95 ns | 3.54 ns | 3.55 ns |
| D153 | 3.79 ns | 3.14 ns | 4.29 ns | 3.46 ns | 4.29 ns |
| D230 | 6 ns | 6.65 ns | 7.24 ns | 5.22 ns | 5.62 ns |
| D307 | 12.3 ns | 11.1 ns | 5.54 ns | 12.5 ns | 12.5 ns |
| D462 | 15.1 ns | 16.7 ns | 16.8 ns | 16.7 ns | 16.6 ns |
| D616 | 19 ns | 22 ns | 20.1 ns | 21.9 ns | 20 ns |
| D924 | 30.9 ns | 54.2 ns | 84.7 ns | 75.8 ns | 96.5 ns |
| D1232 | 47.3 ns | 61.6 ns | 69.8 ns | 52.2 ns | 44 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,181.1 88.2,142.4 124.4,134.7 160.5,131.1 196.7,120.7 232.9,104.8 269.1,91.5 305.3,70.8 341.5,64.8 377.6,58.1 413.8,44.0 450.0,31.7 450.0,33.7 413.8,11.0 377.6,56.5 341.5,61.9 305.3,70.2 269.1,93.4 232.9,101.1 196.7,106.6 160.5,131.1 124.4,127.3 88.2,132.7 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,181.1 88.2,142.4 124.4,134.7 160.5,131.1 196.7,120.7 232.9,104.8 269.1,91.5 305.3,70.8 341.5,64.8 377.6,58.1 413.8,44.0 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,132.7 124.4,127.3 160.5,121.1 196.7,113.0 232.9,110.2 269.1,88.5 305.3,73.7 341.5,61.9 377.6,53.8 413.8,27.7 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.1 124.4,128.3 160.5,121.9 196.7,124.1 232.9,101.2 269.1,86.0 305.3,93.8 341.5,61.7 377.6,56.5 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,135.2 124.4,128.3 160.5,121.0 196.7,106.7 232.9,107.4 269.1,95.5 305.3,70.2 341.5,61.9 377.6,54.0 413.8,18.0 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,131.1 196.7,106.6 232.9,101.1 269.1,93.4 305.3,70.2 341.5,61.9 377.6,56.5 413.8,11.0 450.0,33.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.44 ns | 1.87 ns | 2.11 ns | 2.88 ns | 2.46 ns |
| D38 | 2.34 ns | 3.51 ns | 3.25 ns | 3.18 ns | 3.52 ns |
| D57 | 6.27 ns | 8.09 ns | 7.16 ns | 7.16 ns | 8.09 ns |
| D76 | 5 ns | 9.83 ns | 8.43 ns | 9.83 ns | 4.44 ns |
| D115 | 8 ns | 12.8 ns | 7.42 ns | 14.4 ns | 14.4 ns |
| D153 | 16.2 ns | 11.7 ns | 15.9 ns | 12.9 ns | 17.1 ns |
| D230 | 32.5 ns | 36.2 ns | 36.3 ns | 22.3 ns | 28.3 ns |
| D307 | 48 ns | 40.9 ns | 23.5 ns | 47.9 ns | 47.8 ns |
| D462 | 79.6 ns | 87.9 ns | 89.3 ns | 93.6 ns | 93.3 ns |
| D616 | 83.3 ns | 95.8 ns | 79.3 ns | 95.2 ns | 77.3 ns |
| D924 | 63.9 ns | 77.2 ns | 97 ns | 86.1 ns | 105 ns |
| D1232 | 136 ns | 130 ns | 131 ns | 112 ns | 81.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.4 88.2,185.3 124.4,156.8 160.5,163.4 196.7,149.8 232.9,129.3 269.1,109.2 305.3,97.9 341.5,83.3 377.6,81.9 413.8,89.6 450.0,67.7 450.0,82.7 413.8,75.3 377.6,84.1 341.5,78.7 305.3,98.0 269.1,113.2 232.9,127.9 196.7,132.8 160.5,166.9 124.4,149.5 88.2,173.5 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.4 88.2,185.3 124.4,156.8 160.5,163.4 196.7,149.8 232.9,129.3 269.1,109.2 305.3,97.9 341.5,83.3 377.6,81.9 413.8,89.6 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,173.6 124.4,149.5 160.5,143.8 196.7,136.3 232.9,138.8 269.1,106.1 305.3,102.6 341.5,80.4 377.6,77.9 413.8,84.2 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,175.9 124.4,153.0 160.5,148.3 196.7,152.0 232.9,129.8 269.1,106.0 305.3,118.6 341.5,80.0 377.6,83.4 413.8,77.5 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,176.5 124.4,153.0 160.5,143.8 196.7,132.8 232.9,135.9 269.1,120.1 305.3,98.0 341.5,78.6 377.6,78.1 413.8,81.0 450.0,73.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,173.5 124.4,149.5 160.5,166.9 196.7,132.8 232.9,127.9 269.1,113.2 305.3,98.0 341.5,78.7 377.6,84.1 413.8,75.3 450.0,82.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.843 ns | 0.937 ns | 1.05 ns | 1.15 ns | 1.05 ns |
| D38 | 0.992 ns | 1.82 ns | 1.62 ns | 1.61 ns | 1.82 ns |
| D57 | 1.94 ns | 2.51 ns | 2.25 ns | 2.25 ns | 2.51 ns |
| D76 | 2.09 ns | 3.45 ns | 3.08 ns | 3.46 ns | 1.83 ns |
| D115 | 3.6 ns | 4.83 ns | 3.08 ns | 5.53 ns | 5.56 ns |
| D153 | 7.63 ns | 5.87 ns | 7.63 ns | 6.3 ns | 7.63 ns |
| D230 | 16.1 ns | 17.6 ns | 17.7 ns | 14.8 ns | 13.7 ns |
| D307 | 25.2 ns | 23.4 ns | 13.4 ns | 25.2 ns | 25.2 ns |
| D462 | 46.4 ns | 42.6 ns | 40.6 ns | 50.7 ns | 42.4 ns |
| D616 | 45.7 ns | 58.7 ns | 46.4 ns | 58.6 ns | 45.7 ns |
| D924 | 62.1 ns | 64.1 ns | 84.8 ns | 74.8 ns | 98.6 ns |
| D1232 | 95.6 ns | 95.6 ns | 107 ns | 84 ns | 77.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,163.7 88.2,160.2 124.4,145.6 160.5,144.0 196.7,132.2 232.9,115.9 269.1,99.6 305.3,89.9 341.5,76.7 377.6,77.0 413.8,70.3 450.0,61.0 450.0,65.5 413.8,60.3 377.6,77.0 341.5,78.6 305.3,89.9 269.1,103.2 232.9,115.9 196.7,122.7 160.5,146.9 124.4,140.0 88.2,147.0 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,163.7 88.2,160.2 124.4,145.6 160.5,144.0 196.7,132.2 232.9,115.9 269.1,99.6 305.3,89.9 341.5,76.7 377.6,77.0 413.8,70.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,147.0 124.4,140.0 160.5,133.1 196.7,125.8 232.9,121.6 269.1,97.7 305.3,91.5 341.5,78.5 377.6,71.6 413.8,69.7 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,142.4 160.5,135.6 196.7,135.6 232.9,115.9 269.1,97.6 305.3,103.6 341.5,79.6 377.6,76.7 413.8,63.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,149.7 124.4,142.4 160.5,133.1 196.7,122.9 232.9,120.0 269.1,101.5 305.3,90.0 341.5,74.8 377.6,71.6 413.8,66.3 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.0 160.5,146.9 196.7,122.7 232.9,115.9 269.1,103.2 305.3,89.9 341.5,78.6 377.6,77.0 413.8,60.3 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
