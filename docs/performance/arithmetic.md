# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.939 ns | 1.06 ns | 1.06 ns | 0.818 ns | 1.05 ns |
| D38 | 1.63 ns | 1.42 ns | 1.83 ns | 1.82 ns | 1.44 ns |
| D57 | 2.5 ns | 2.25 ns | 2.5 ns | 2.49 ns | 2.25 ns |
| D76 | 3.08 ns | 1.62 ns | 3.5 ns | 3.09 ns | 3.08 ns |
| D115 | 4.4 ns | 3.26 ns | 4.42 ns | 4.4 ns | 3.33 ns |
| D153 | 5.91 ns | 4.41 ns | 6.62 ns | 5.64 ns | 5.9 ns |
| D230 | 8.32 ns | 15.3 ns | 13.9 ns | 11.9 ns | 15.3 ns |
| D307 | 11.8 ns | 19.6 ns | 18.6 ns | 14.4 ns | 19.6 ns |
| D462 | 29 ns | 29.9 ns | 33.4 ns | 35.2 ns | 33.1 ns |
| D616 | 42.3 ns | 43.3 ns | 65.7 ns | 48.7 ns | 51.1 ns |
| D924 | 98.4 ns | 74.5 ns | 92.2 ns | 74.4 ns | 98.4 ns |
| D1232 | 95.1 ns | 109 ns | 107 ns | 90.3 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.3 124.4,140.1 160.5,135.6 196.7,127.8 232.9,121.4 269.1,114.0 305.3,106.4 341.5,86.9 377.6,78.7 413.8,60.3 450.0,61.1 450.0,58.5 413.8,60.3 377.6,74.6 341.5,84.0 305.3,95.4 269.1,100.7 232.9,121.5 196.7,133.9 160.5,135.6 124.4,142.4 88.2,152.0 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.3 124.4,140.1 160.5,135.6 196.7,127.8 232.9,121.4 269.1,114.0 305.3,106.4 341.5,86.9 377.6,78.7 413.8,60.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.7 88.2,152.4 124.4,142.4 160.5,149.5 196.7,134.4 232.9,127.8 269.1,100.8 305.3,95.4 341.5,86.2 377.6,78.2 413.8,66.4 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,146.9 124.4,140.1 160.5,132.8 196.7,127.7 232.9,118.9 269.1,102.9 305.3,96.6 341.5,83.8 377.6,69.1 413.8,61.8 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.4 88.2,147.0 124.4,140.2 160.5,135.5 196.7,127.8 232.9,122.4 269.1,106.2 305.3,102.1 341.5,82.7 377.6,75.6 413.8,66.4 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.0 124.4,142.4 160.5,135.6 196.7,133.9 232.9,121.5 269.1,100.7 305.3,95.4 341.5,84.0 377.6,74.6 413.8,60.3 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.77 ns | 6.2 ns | 6.2 ns | 6.82 ns | 8.95 ns |
| D38 | 9.73 ns | 9.8 ns | 14.3 ns | 67.8 ns | 65.4 ns |
| D57 | 22.6 ns | 33.3 ns | 74.8 ns | 121 ns | 114 ns |
| D76 | 26.2 ns | 33.1 ns | 84.1 ns | 115 ns | 141 ns |
| D115 | 42.8 ns | 62.4 ns | 104 ns | 186 ns | 231 ns |
| D153 | 55 ns | 87 ns | 154 ns | 237 ns | 322 ns |
| D230 | 45.9 ns | 173 ns | 233 ns | 341 ns | 625 ns |
| D307 | 66.1 ns | 253 ns | 359 ns | 479 ns | 972 ns |
| D462 | 197 ns | 417 ns | 747 ns | 1.06 µs | 1.49 µs |
| D616 | 190 ns | 379 ns | 1.14 µs | 1.84 µs | 2.4 µs |
| D924 | 402 ns | 1.09 µs | 1.83 µs | 2.54 µs | 4.69 µs |
| D1232 | 507 ns | 1.94 µs | 3.81 µs | 3.65 µs | 7.84 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.9 88.2,160.6 124.4,142.3 160.5,139.1 196.7,128.4 232.9,123.0 269.1,126.9 305.3,119.0 341.5,95.3 377.6,96.1 413.8,79.8 450.0,74.8 450.0,15.3 413.8,26.4 377.6,41.0 341.5,51.4 305.3,60.6 269.1,70.2 232.9,84.6 196.7,91.9 160.5,102.5 124.4,107.1 88.2,119.2 52.0,162.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.9 88.2,160.6 124.4,142.3 160.5,139.1 196.7,128.4 232.9,123.0 269.1,126.9 305.3,119.0 341.5,95.3 377.6,96.1 413.8,79.8 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,160.4 124.4,133.9 160.5,134.0 196.7,120.3 232.9,113.0 269.1,98.1 305.3,89.9 341.5,79.0 377.6,81.1 413.8,58.1 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,152.2 124.4,116.3 160.5,113.8 196.7,109.2 232.9,100.6 269.1,91.7 305.3,82.2 341.5,66.3 377.6,57.2 413.8,46.9 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.3 88.2,118.4 124.4,105.9 160.5,107.0 196.7,96.5 232.9,91.3 269.1,83.4 305.3,76.0 341.5,58.7 377.6,46.8 413.8,39.7 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.4 88.2,119.2 124.4,107.1 160.5,102.5 196.7,91.9 232.9,84.6 269.1,70.2 305.3,60.6 341.5,51.4 377.6,41.0 413.8,26.4 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 3.07 ns | 3.19 ns | 3.9 ns | 4.98 ns |
| D38 | 3.49 ns | 10.7 ns | 28.7 ns | 42.3 ns | 24.9 ns |
| D57 | 4.22 ns | 21.4 ns | 35.2 ns | 76.8 ns | 71.9 ns |
| D76 | 7.79 ns | 21.1 ns | 45.4 ns | 78.9 ns | 102 ns |
| D115 | 13.3 ns | 34.4 ns | 89.2 ns | 195 ns | 214 ns |
| D153 | 18.1 ns | 38.6 ns | 121 ns | 228 ns | 356 ns |
| D230 | 14 ns | 133 ns | 343 ns | 442 ns | 1.02 µs |
| D307 | 22.4 ns | 187 ns | 465 ns | 741 ns | 1.47 µs |
| D462 | 81.6 ns | 408 ns | 1.33 µs | 1.67 µs | 2.63 µs |
| D616 | 77.6 ns | 414 ns | 1.89 µs | 2.73 µs | 4.19 µs |
| D924 | 163 ns | 1.47 µs | 2.57 µs | 4.98 µs | 8.34 µs |
| D1232 | 192 ns | 2.38 µs | 5.13 µs | 7.01 µs | 14.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.6 124.4,155.8 160.5,147.0 196.7,139.3 232.9,134.7 269.1,138.5 305.3,131.6 341.5,112.9 377.6,113.7 413.8,102.9 450.0,100.6 450.0,38.3 413.8,46.0 377.6,55.9 341.5,62.7 305.3,71.1 269.1,76.3 232.9,91.6 196.7,99.0 160.5,109.7 124.4,114.8 88.2,130.1 52.0,153.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.6 124.4,155.8 160.5,147.0 196.7,139.3 232.9,134.7 269.1,138.5 305.3,131.6 341.5,112.9 377.6,113.7 413.8,102.9 450.0,100.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.4 88.2,142.4 124.4,132.3 160.5,132.5 196.7,125.5 232.9,123.8 269.1,105.8 305.3,101.0 341.5,89.6 377.6,89.5 413.8,71.1 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.9 88.2,128.1 124.4,125.1 160.5,121.4 196.7,111.7 232.9,107.3 269.1,92.2 305.3,87.7 341.5,72.5 377.6,67.5 413.8,63.0 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,122.5 124.4,113.8 160.5,113.4 196.7,100.3 232.9,98.1 269.1,88.5 305.3,81.0 341.5,69.3 377.6,62.1 413.8,53.4 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,130.1 124.4,114.8 160.5,109.7 196.7,99.0 232.9,91.6 269.1,76.3 305.3,71.1 341.5,62.7 377.6,55.9 413.8,46.0 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.703 ns | 0.703 ns | 0.545 ns | 0.703 ns |
| D38 | 1.33 ns | 1.12 ns | 1.45 ns | 1.45 ns | 1.15 ns |
| D57 | 1.74 ns | 1.87 ns | 1.74 ns | 1.74 ns | 1.88 ns |
| D76 | 2.09 ns | 1.47 ns | 2.16 ns | 2.09 ns | 2.49 ns |
| D115 | 2.83 ns | 2.46 ns | 2.83 ns | 3.28 ns | 2.78 ns |
| D153 | 3.82 ns | 3.18 ns | 4.6 ns | 4.02 ns | 4.29 ns |
| D230 | 3.08 ns | 6.65 ns | 7.16 ns | 5.62 ns | 7.24 ns |
| D307 | 4.95 ns | 12.4 ns | 11 ns | 7.03 ns | 12.4 ns |
| D462 | 15.1 ns | 21.9 ns | 17 ns | 22.2 ns | 17 ns |
| D616 | 15.7 ns | 15.4 ns | 21.8 ns | 19.8 ns | 21.9 ns |
| D924 | 74.5 ns | 74.9 ns | 82.7 ns | 75.3 ns | 96.6 ns |
| D1232 | 47.5 ns | 69.7 ns | 69.8 ns | 63.5 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,135.1 124.4,127.3 160.5,121.9 196.7,113.2 232.9,104.5 269.1,110.8 305.3,97.0 341.5,64.8 377.6,63.6 413.8,18.5 450.0,31.5 450.0,20.4 413.8,11.0 377.6,54.0 341.5,61.4 305.3,70.5 269.1,86.0 232.9,101.1 196.7,113.7 160.5,117.0 124.4,125.0 88.2,139.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,135.1 124.4,127.3 160.5,121.9 196.7,113.2 232.9,104.5 269.1,110.8 305.3,97.0 341.5,64.8 377.6,63.6 413.8,18.5 450.0,31.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,140.0 124.4,125.2 160.5,132.2 196.7,117.2 232.9,109.8 269.1,88.5 305.3,70.5 341.5,54.0 377.6,64.1 413.8,18.4 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,121.0 196.7,113.2 232.9,99.1 269.1,86.3 305.3,73.9 341.5,61.3 377.6,54.1 413.8,15.5 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,132.7 124.4,127.3 160.5,122.0 196.7,108.9 232.9,103.1 269.1,93.4 305.3,86.9 341.5,53.6 377.6,56.8 413.8,18.2 450.0,23.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,139.2 124.4,125.0 160.5,117.0 196.7,113.7 232.9,101.1 269.1,86.0 305.3,70.5 341.5,61.4 377.6,54.0 413.8,11.0 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 2.11 ns | 1.91 ns | 2.46 ns |
| D38 | 4.36 ns | 3.65 ns | 3.51 ns | 3.54 ns | 3.54 ns |
| D57 | 8.08 ns | 7.17 ns | 8.09 ns | 8.09 ns | 7.2 ns |
| D76 | 8.44 ns | 4.5 ns | 9.84 ns | 8.72 ns | 8.44 ns |
| D115 | 12.7 ns | 9.07 ns | 12.7 ns | 12.8 ns | 9.15 ns |
| D153 | 16.1 ns | 12.7 ns | 20 ns | 17.7 ns | 17 ns |
| D230 | 17.4 ns | 36 ns | 32 ns | 27.9 ns | 36.6 ns |
| D307 | 24.6 ns | 48 ns | 43 ns | 28.2 ns | 47.8 ns |
| D462 | 80.1 ns | 74.2 ns | 91.3 ns | 70.7 ns | 83 ns |
| D616 | 68.3 ns | 60 ns | 93.8 ns | 77.6 ns | 92.1 ns |
| D924 | 126 ns | 105 ns | 118 ns | 92.4 ns | 106 ns |
| D1232 | 133 ns | 136 ns | 128 ns | 106 ns | 121 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,167.4 124.4,149.5 160.5,148.3 196.7,136.3 232.9,129.6 269.1,127.2 305.3,117.2 341.5,83.1 377.6,87.7 413.8,69.9 450.0,68.4 450.0,71.2 413.8,75.0 377.6,79.1 341.5,82.0 305.3,98.0 269.1,105.8 232.9,128.0 196.7,145.9 160.5,148.3 124.4,152.8 88.2,173.4 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,167.4 124.4,149.5 160.5,148.3 196.7,136.3 232.9,129.6 269.1,127.2 305.3,117.2 341.5,83.1 377.6,87.7 413.8,69.9 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,172.5 124.4,153.0 160.5,166.5 196.7,146.2 232.9,136.5 269.1,106.2 305.3,97.9 341.5,85.3 377.6,91.4 413.8,75.3 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,173.6 124.4,149.5 160.5,143.8 196.7,136.3 232.9,123.2 269.1,109.6 305.3,101.1 341.5,79.3 377.6,78.5 413.8,71.8 450.0,69.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.3 88.2,173.4 124.4,149.5 160.5,147.3 196.7,136.3 232.9,126.8 269.1,113.6 305.3,113.3 341.5,86.7 377.6,84.0 413.8,79.0 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,173.4 124.4,152.8 160.5,148.3 196.7,145.9 232.9,128.0 269.1,105.8 305.3,98.0 341.5,82.0 377.6,79.1 413.8,75.0 450.0,71.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 1.05 ns | 1.06 ns | 0.819 ns | 1.06 ns |
| D38 | 1.63 ns | 1.42 ns | 1.81 ns | 1.8 ns | 1.45 ns |
| D57 | 2.5 ns | 2.27 ns | 2.5 ns | 2.49 ns | 2.27 ns |
| D76 | 3.09 ns | 1.83 ns | 3.46 ns | 3.09 ns | 3.09 ns |
| D115 | 4.89 ns | 4.17 ns | 4.9 ns | 4.88 ns | 4.03 ns |
| D153 | 7.54 ns | 5.97 ns | 8.46 ns | 7.58 ns | 7.55 ns |
| D230 | 9.73 ns | 17.6 ns | 16.1 ns | 13.6 ns | 17.7 ns |
| D307 | 13.5 ns | 24.9 ns | 23.1 ns | 16.4 ns | 24.9 ns |
| D462 | 37.7 ns | 38.2 ns | 43.3 ns | 37.4 ns | 40.7 ns |
| D616 | 47.3 ns | 45.4 ns | 65.6 ns | 49.8 ns | 49.1 ns |
| D924 | 98.4 ns | 74.8 ns | 92.2 ns | 74.8 ns | 98.3 ns |
| D1232 | 95.7 ns | 107 ns | 106 ns | 90.6 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,149.4 124.4,140.1 160.5,135.5 196.7,125.5 232.9,116.1 269.1,110.6 305.3,103.5 341.5,81.2 377.6,76.2 413.8,60.3 450.0,61.0 450.0,58.7 413.8,60.4 377.6,75.4 341.5,79.5 305.3,90.2 269.1,97.6 232.9,116.1 196.7,129.7 160.5,135.5 124.4,142.2 88.2,151.9 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,149.4 124.4,140.1 160.5,135.5 196.7,125.5 232.9,116.1 269.1,110.6 305.3,103.5 341.5,81.2 377.6,76.2 413.8,60.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,152.5 124.4,142.2 160.5,146.9 196.7,129.0 232.9,121.2 269.1,97.7 305.3,90.2 341.5,80.9 377.6,77.1 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,140.1 160.5,133.1 196.7,125.5 232.9,113.6 269.1,99.6 305.3,91.8 341.5,78.2 377.6,69.2 413.8,61.8 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,147.2 124.4,140.2 160.5,135.5 196.7,125.6 232.9,116.0 269.1,103.3 305.3,99.3 341.5,81.3 377.6,75.1 413.8,66.3 450.0,62.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,151.9 124.4,142.2 160.5,135.5 196.7,129.7 232.9,116.1 269.1,97.6 305.3,90.2 341.5,79.5 377.6,75.4 413.8,60.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
