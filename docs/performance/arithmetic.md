# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.819 ns | 1.06 ns | 0.935 ns | 0.941 ns | 0.935 ns |
| D38 | 1.61 ns | 1.61 ns | 1.63 ns | 1.82 ns | 1.61 ns |
| D57 | 2.5 ns | 2.49 ns | 1.93 ns | 2.88 ns | 2.25 ns |
| D76 | 2.08 ns | 3.49 ns | 3.48 ns | 3.49 ns | 3.09 ns |
| D115 | 4.99 ns | 4.4 ns | 5 ns | 4.97 ns | 3.33 ns |
| D153 | 6.64 ns | 3.85 ns | 3.36 ns | 6.63 ns | 5.15 ns |
| D230 | 14 ns | 15.3 ns | 15.3 ns | 14 ns | 15.4 ns |
| D307 | 15.1 ns | 15.2 ns | 18.5 ns | 18.5 ns | 19.5 ns |
| D462 | 28.9 ns | 26.4 ns | 25.2 ns | 29.2 ns | 28.9 ns |
| D616 | 45.4 ns | 51.5 ns | 45.1 ns | 34.2 ns | 50.1 ns |
| D924 | 74.5 ns | 75.5 ns | 98.7 ns | 73.8 ns | 85.4 ns |
| D1232 | 107 ns | 107 ns | 94.9 ns | 68.7 ns | 95.3 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.3 88.2,149.6 124.4,140.1 160.5,144.1 196.7,125.1 232.9,118.9 269.1,102.7 305.3,101.0 341.5,87.0 377.6,77.2 413.8,66.4 450.0,58.5 450.0,61.1 413.8,63.4 377.6,75.0 341.5,86.9 305.3,95.5 269.1,100.7 232.9,124.4 196.7,133.9 160.5,135.5 124.4,142.4 88.2,149.7 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.3 88.2,149.6 124.4,140.1 160.5,144.1 196.7,125.1 232.9,118.9 269.1,102.7 305.3,101.0 341.5,87.0 377.6,77.2 413.8,66.4 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.6 124.4,140.2 160.5,132.9 196.7,127.8 232.9,130.7 269.1,100.8 305.3,100.9 341.5,88.9 377.6,74.4 413.8,66.1 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.4 124.4,145.7 160.5,132.9 196.7,125.1 232.9,133.7 269.1,100.8 305.3,96.6 341.5,89.9 377.6,77.3 413.8,60.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,147.0 124.4,137.0 160.5,132.9 196.7,125.2 232.9,118.9 269.1,102.7 305.3,96.6 341.5,86.7 377.6,83.3 413.8,66.6 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.7 124.4,142.4 160.5,135.5 196.7,133.9 232.9,124.4 269.1,100.7 305.3,95.5 341.5,86.9 377.6,75.0 413.8,63.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5 ns | 6.2 ns | 5.98 ns | 8.19 ns | 7.85 ns |
| D38 | 9.78 ns | 11.4 ns | 15.4 ns | 67.4 ns | 59.7 ns |
| D57 | 22.9 ns | 34.7 ns | 58.1 ns | 113 ns | 114 ns |
| D76 | 21.6 ns | 65.4 ns | 83.8 ns | 120 ns | 142 ns |
| D115 | 46.4 ns | 104 ns | 112 ns | 201 ns | 228 ns |
| D153 | 57.4 ns | 76.8 ns | 90.7 ns | 266 ns | 274 ns |
| D230 | 60.6 ns | 171 ns | 252 ns | 394 ns | 625 ns |
| D307 | 103 ns | 192 ns | 365 ns | 596 ns | 973 ns |
| D462 | 200 ns | 280 ns | 438 ns | 1.06 µs | 1.31 µs |
| D616 | 241 ns | 673 ns | 1.04 µs | 1.2 µs | 1.89 µs |
| D924 | 352 ns | 1.09 µs | 2.29 µs | 2.25 µs | 4.7 µs |
| D1232 | 537 ns | 1.95 µs | 3.53 µs | 3.26 µs | 6.97 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.0 88.2,160.5 124.4,142.0 160.5,143.3 196.7,126.7 232.9,122.1 269.1,120.9 305.3,109.4 341.5,94.9 377.6,90.9 413.8,82.7 450.0,73.5 450.0,17.8 413.8,26.4 377.6,46.1 341.5,54.2 305.3,60.6 269.1,70.2 232.9,88.1 196.7,92.1 160.5,102.3 124.4,107.2 88.2,121.2 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.0 88.2,160.5 124.4,142.0 160.5,143.3 196.7,126.7 232.9,122.1 269.1,120.9 305.3,109.4 341.5,94.9 377.6,90.9 413.8,82.7 450.0,73.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,157.1 124.4,133.0 160.5,119.2 196.7,109.1 232.9,115.7 269.1,98.4 305.3,95.8 341.5,87.7 377.6,68.6 413.8,58.2 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.2 88.2,150.7 124.4,121.8 160.5,113.8 196.7,107.4 232.9,112.1 269.1,89.9 305.3,81.9 341.5,77.9 377.6,59.2 413.8,42.0 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,118.6 124.4,107.4 160.5,106.0 196.7,94.8 232.9,88.8 269.1,80.2 305.3,71.2 341.5,58.8 377.6,56.0 413.8,42.4 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,121.2 124.4,107.2 160.5,102.3 196.7,92.1 232.9,88.1 269.1,70.2 305.3,60.6 341.5,54.2 377.6,46.1 413.8,26.4 450.0,17.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.816 ns | 3.07 ns | 3.1 ns | 4.93 ns | 5.12 ns |
| D38 | 3.5 ns | 14.7 ns | 26.2 ns | 38 ns | 27.8 ns |
| D57 | 4.22 ns | 21.6 ns | 26.7 ns | 72.2 ns | 71.8 ns |
| D76 | 5.2 ns | 38.1 ns | 44.9 ns | 83.3 ns | 102 ns |
| D115 | 13.6 ns | 49.3 ns | 93.4 ns | 215 ns | 214 ns |
| D153 | 16.8 ns | 34.2 ns | 65.2 ns | 259 ns | 308 ns |
| D230 | 21.4 ns | 132 ns | 368 ns | 524 ns | 1.05 µs |
| D307 | 42.2 ns | 143 ns | 450 ns | 1.04 µs | 1.47 µs |
| D462 | 97.6 ns | 266 ns | 670 ns | 1.75 µs | 2.46 µs |
| D616 | 92.8 ns | 727 ns | 1.75 µs | 1.76 µs | 3.29 µs |
| D924 | 144 ns | 1.47 µs | 3.22 µs | 4.27 µs | 8.34 µs |
| D1232 | 200 ns | 2.36 µs | 4.71 µs | 5.77 µs | 13.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,158.5 124.4,155.8 160.5,152.8 196.7,138.9 232.9,135.8 269.1,132.3 305.3,122.5 341.5,110.4 377.6,111.1 413.8,104.7 450.0,100.0 450.0,39.2 413.8,46.0 377.6,59.4 341.5,63.6 305.3,71.1 269.1,75.9 232.9,93.7 196.7,99.0 160.5,109.7 124.4,114.8 88.2,128.5 52.0,153.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,158.5 124.4,155.8 160.5,152.8 196.7,138.9 232.9,135.8 269.1,132.3 305.3,122.5 341.5,110.4 377.6,111.1 413.8,104.7 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.4 88.2,137.8 124.4,132.2 160.5,124.0 196.7,120.2 232.9,125.6 269.1,106.0 305.3,104.8 341.5,95.8 377.6,81.3 413.8,71.1 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,129.4 124.4,129.1 160.5,121.6 196.7,111.0 232.9,116.2 269.1,91.2 305.3,88.2 341.5,82.5 377.6,68.6 413.8,59.7 450.0,54.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,124.0 124.4,114.7 160.5,112.6 196.7,98.9 232.9,96.2 269.1,86.0 305.3,76.2 341.5,68.5 377.6,68.5 413.8,55.6 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.0 88.2,128.5 124.4,114.8 160.5,109.7 196.7,99.0 232.9,93.7 269.1,75.9 305.3,71.1 341.5,63.6 377.6,59.4 413.8,46.0 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.545 ns | 0.703 ns | 0.622 ns | 0.622 ns | 0.622 ns |
| D38 | 1.33 ns | 1.33 ns | 1.32 ns | 1.45 ns | 1.32 ns |
| D57 | 1.74 ns | 1.74 ns | 1.35 ns | 1.57 ns | 1.68 ns |
| D76 | 1.69 ns | 2.16 ns | 2.16 ns | 2.16 ns | 2.5 ns |
| D115 | 3.17 ns | 2.85 ns | 3.17 ns | 3.55 ns | 2.78 ns |
| D153 | 4.22 ns | 2.78 ns | 2.45 ns | 4.6 ns | 3.57 ns |
| D230 | 5.21 ns | 6.65 ns | 7.43 ns | 7.18 ns | 7.44 ns |
| D307 | 9.51 ns | 9.59 ns | 11.1 ns | 11.1 ns | 12.4 ns |
| D462 | 15 ns | 12.3 ns | 10.6 ns | 15.4 ns | 15.4 ns |
| D616 | 19 ns | 21.9 ns | 20.3 ns | 15.4 ns | 17.8 ns |
| D924 | 55 ns | 76.5 ns | 93.7 ns | 69.3 ns | 84.8 ns |
| D1232 | 54.4 ns | 69.8 ns | 61.9 ns | 43.4 ns | 61.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.9 88.2,135.2 124.4,127.3 160.5,128.1 196.7,110.0 232.9,101.6 269.1,95.6 305.3,78.1 341.5,65.0 377.6,58.0 413.8,27.3 450.0,27.6 450.0,23.9 413.8,14.8 377.6,60.0 341.5,64.2 305.3,70.5 269.1,85.2 232.9,106.5 196.7,113.7 160.5,116.9 124.4,128.3 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.9 88.2,135.2 124.4,127.3 160.5,128.1 196.7,110.0 232.9,101.6 269.1,95.6 305.3,78.1 341.5,65.0 377.6,58.0 413.8,27.3 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,121.1 196.7,113.0 232.9,113.7 269.1,88.5 305.3,77.9 341.5,70.6 377.6,54.0 413.8,17.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,134.6 160.5,121.0 196.7,109.9 232.9,117.4 269.1,85.3 305.3,73.7 341.5,75.1 377.6,56.2 413.8,11.9 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,130.3 160.5,121.0 196.7,106.7 232.9,99.1 269.1,86.3 305.3,73.7 341.5,64.2 377.6,64.2 413.8,20.6 450.0,34.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,128.3 160.5,116.9 196.7,113.7 232.9,106.5 269.1,85.2 305.3,70.5 341.5,64.2 377.6,60.0 413.8,14.8 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 ns | 2.11 ns | 1.87 ns | 2.18 ns | 2.18 ns |
| D38 | 3.31 ns | 3.31 ns | 3.31 ns | 3.54 ns | 3.19 ns |
| D57 | 8.09 ns | 8.09 ns | 6.28 ns | 6.16 ns | 7.16 ns |
| D76 | 6.29 ns | 9.84 ns | 9.83 ns | 9.5 ns | 8.52 ns |
| D115 | 14.4 ns | 12.8 ns | 14.4 ns | 14.4 ns | 9.48 ns |
| D153 | 20 ns | 11.1 ns | 9.09 ns | 20 ns | 15.6 ns |
| D230 | 23.5 ns | 36.3 ns | 36 ns | 31.8 ns | 36.4 ns |
| D307 | 37.3 ns | 37.3 ns | 42.9 ns | 42.7 ns | 47.8 ns |
| D462 | 75 ns | 48.3 ns | 43.4 ns | 73 ns | 70 ns |
| D616 | 82.9 ns | 101 ns | 78.9 ns | 54.4 ns | 70.4 ns |
| D924 | 106 ns | 104 ns | 96.6 ns | 99.2 ns | 92.7 ns |
| D1232 | 150 ns | 135 ns | 120 ns | 74.2 ns | 111 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.8 88.2,175.3 124.4,149.5 160.5,156.7 196.7,132.8 232.9,123.2 269.1,118.6 305.3,105.2 341.5,85.0 377.6,82.1 413.8,74.9 450.0,64.8 450.0,73.6 413.8,78.9 377.6,86.8 341.5,87.0 305.3,98.0 269.1,105.9 232.9,130.5 196.7,144.9 160.5,148.0 124.4,153.0 88.2,176.4 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.8 88.2,175.3 124.4,149.5 160.5,156.7 196.7,132.8 232.9,123.2 269.1,118.6 305.3,105.2 341.5,85.0 377.6,82.1 413.8,74.9 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,175.3 124.4,149.5 160.5,143.8 196.7,136.2 232.9,140.3 269.1,106.0 305.3,105.2 341.5,97.8 377.6,76.3 413.8,75.4 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,175.4 124.4,156.8 160.5,143.8 196.7,132.8 232.9,146.1 269.1,106.3 305.3,101.2 341.5,100.8 377.6,83.5 413.8,77.7 450.0,71.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,173.4 124.4,157.4 160.5,144.8 196.7,132.8 232.9,123.2 269.1,109.8 305.3,101.3 341.5,85.8 377.6,94.3 413.8,76.9 450.0,85.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.4 124.4,153.0 160.5,148.0 196.7,144.9 232.9,130.5 269.1,105.9 305.3,98.0 341.5,87.0 377.6,86.8 413.8,78.9 450.0,73.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.818 ns | 1.05 ns | 0.938 ns | 0.942 ns | 0.935 ns |
| D38 | 1.61 ns | 1.61 ns | 1.61 ns | 1.81 ns | 1.6 ns |
| D57 | 2.5 ns | 2.5 ns | 1.94 ns | 2.94 ns | 2.25 ns |
| D76 | 2.42 ns | 3.45 ns | 3.46 ns | 3.46 ns | 3.1 ns |
| D115 | 5.56 ns | 4.89 ns | 5.56 ns | 5.55 ns | 4.03 ns |
| D153 | 8.44 ns | 5.24 ns | 4.66 ns | 8.44 ns | 6.57 ns |
| D230 | 15.5 ns | 17.6 ns | 17.6 ns | 16.1 ns | 17.6 ns |
| D307 | 19.5 ns | 19.5 ns | 23.5 ns | 23.4 ns | 25.2 ns |
| D462 | 36.9 ns | 33.5 ns | 29.7 ns | 37.1 ns | 37.1 ns |
| D616 | 46.2 ns | 49 ns | 45.9 ns | 37.5 ns | 50.1 ns |
| D924 | 75.3 ns | 77.4 ns | 97.9 ns | 73.1 ns | 84.9 ns |
| D1232 | 106 ns | 106 ns | 95 ns | 83.4 ns | 95 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.4 88.2,149.6 124.4,140.1 160.5,140.8 196.7,122.8 232.9,113.7 269.1,100.5 305.3,95.5 341.5,81.6 377.6,76.8 413.8,66.2 450.0,58.7 450.0,61.1 413.8,63.6 377.6,75.0 341.5,81.5 305.3,90.0 269.1,97.7 232.9,119.1 196.7,129.8 160.5,135.4 124.4,142.4 88.2,149.8 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.4 88.2,149.6 124.4,140.1 160.5,140.8 196.7,122.8 232.9,113.7 269.1,100.5 305.3,95.5 341.5,81.6 377.6,76.8 413.8,66.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.9 88.2,149.6 124.4,140.1 160.5,133.1 196.7,125.5 232.9,124.0 269.1,97.7 305.3,95.5 341.5,83.8 377.6,75.5 413.8,65.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,145.6 160.5,133.1 196.7,122.7 232.9,126.6 269.1,97.7 305.3,91.4 341.5,86.4 377.6,76.9 413.8,60.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,147.1 124.4,136.6 160.5,133.0 196.7,122.8 232.9,113.7 269.1,99.6 305.3,91.5 341.5,81.5 377.6,81.3 413.8,66.8 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.8 124.4,142.4 160.5,135.4 196.7,129.8 232.9,119.1 269.1,97.7 305.3,90.0 341.5,81.5 377.6,75.0 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
