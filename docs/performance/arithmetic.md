# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 0.936 ns | 1.05 ns | 1.17 ns | 0.934 ns |
| D38 | 1.81 ns | 1.82 ns | 1.83 ns | 1.82 ns | 1.82 ns |
| D57 | 2.28 ns | 2.88 ns | 2.28 ns | 2.28 ns | 1.72 ns |
| D76 | 2.32 ns | 3.49 ns | 3.07 ns | 2.71 ns | 3.1 ns |
| D115 | 4.99 ns | 4.42 ns | 4.39 ns | 4.4 ns | 5.83 ns |
| D153 | 5.89 ns | 5.9 ns | 5.89 ns | 5.88 ns | 5.89 ns |
| D230 | 15.3 ns | 13.9 ns | 10.2 ns | 8.4 ns | 13.9 ns |
| D307 | 18.5 ns | 19.6 ns | 19.6 ns | 18.5 ns | 14.6 ns |
| D462 | 28.6 ns | 30.2 ns | 32.5 ns | 23 ns | 33.4 ns |
| D616 | 45.4 ns | 33.9 ns | 45 ns | 48.7 ns | 45.8 ns |
| D924 | 74.4 ns | 74.7 ns | 75.1 ns | 75.5 ns | 61.1 ns |
| D1232 | 106 ns | 105 ns | 106 ns | 106 ns | 92.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.7 88.2,147.1 124.4,142.1 160.5,141.8 196.7,125.1 232.9,121.5 269.1,100.7 305.3,96.6 341.5,87.2 377.6,77.1 413.8,66.4 450.0,58.8 450.0,61.7 413.8,70.7 377.6,77.0 341.5,83.8 305.3,101.7 269.1,102.9 232.9,121.5 196.7,121.7 160.5,135.4 124.4,148.2 88.2,147.0 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.7 88.2,147.1 124.4,142.1 160.5,141.8 196.7,125.1 232.9,121.5 269.1,100.7 305.3,96.6 341.5,87.2 377.6,77.1 413.8,66.4 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,147.0 124.4,137.0 160.5,132.9 196.7,127.7 232.9,121.5 269.1,102.8 305.3,95.4 341.5,86.0 377.6,83.5 413.8,66.3 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.1 160.5,135.6 196.7,127.9 232.9,121.5 269.1,109.6 305.3,95.4 341.5,84.4 377.6,77.3 413.8,66.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.7 88.2,147.0 124.4,142.1 160.5,138.3 196.7,127.8 232.9,121.5 269.1,113.8 305.3,96.6 341.5,92.0 377.6,75.6 413.8,66.1 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,148.2 160.5,135.4 196.7,121.7 232.9,121.5 269.1,102.9 305.3,101.7 341.5,83.8 377.6,77.0 413.8,70.7 450.0,61.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.49 ns | 5.93 ns | 6.2 ns | 8.65 ns | 7.88 ns |
| D38 | 12.5 ns | 12.7 ns | 14.6 ns | 66.9 ns | 68.3 ns |
| D57 | 24 ns | 28.5 ns | 67.7 ns | 114 ns | 96.2 ns |
| D76 | 21.8 ns | 65.2 ns | 75.7 ns | 100 ns | 142 ns |
| D115 | 49.7 ns | 82.6 ns | 104 ns | 187 ns | 296 ns |
| D153 | 54.9 ns | 109 ns | 151 ns | 240 ns | 321 ns |
| D230 | 94.5 ns | 155 ns | 144 ns | 260 ns | 575 ns |
| D307 | 116 ns | 247 ns | 399 ns | 585 ns | 716 ns |
| D462 | 191 ns | 306 ns | 764 ns | 1.03 µs | 1.48 µs |
| D616 | 235 ns | 468 ns | 1.04 µs | 1.37 µs | 2.16 µs |
| D924 | 353 ns | 1.09 µs | 2.05 µs | 2.54 µs | 3.54 µs |
| D1232 | 537 ns | 1.77 µs | 3.82 µs | 4.66 µs | 6.83 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,155.2 124.4,141.0 160.5,143.1 196.7,125.2 232.9,123.0 269.1,111.2 305.3,106.7 341.5,96.0 377.6,91.4 413.8,82.6 450.0,73.5 450.0,18.3 413.8,32.5 377.6,43.3 341.5,51.5 305.3,67.3 269.1,72.0 232.9,84.7 196.7,86.5 160.5,102.3 124.4,110.8 88.2,118.3 52.0,165.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,155.2 124.4,141.0 160.5,143.1 196.7,125.2 232.9,123.0 269.1,111.2 305.3,106.7 341.5,96.0 377.6,91.4 413.8,82.6 450.0,73.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.3 88.2,154.9 124.4,137.3 160.5,119.3 196.7,114.2 232.9,108.2 269.1,100.5 305.3,90.4 341.5,85.7 377.6,76.5 413.8,58.2 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,151.7 124.4,118.5 160.5,116.0 196.7,109.2 232.9,101.0 269.1,102.0 305.3,80.0 341.5,65.9 377.6,59.1 413.8,44.4 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.1 88.2,118.7 124.4,107.2 160.5,110.0 196.7,96.4 232.9,91.0 269.1,89.3 305.3,71.7 341.5,59.5 377.6,53.2 413.8,39.7 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.2 88.2,118.3 124.4,110.8 160.5,102.3 196.7,86.5 232.9,84.7 269.1,72.0 305.3,67.3 341.5,51.5 377.6,43.3 413.8,32.5 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 2.87 ns | 3.2 ns | 4.38 ns | 5.05 ns |
| D38 | 3.94 ns | 13.7 ns | 37.5 ns | 30.2 ns | 48.3 ns |
| D57 | 6.55 ns | 20.4 ns | 32.9 ns | 71.8 ns | 62.5 ns |
| D76 | 5.4 ns | 38.1 ns | 42.3 ns | 65.4 ns | 102 ns |
| D115 | 13.6 ns | 49.8 ns | 84.4 ns | 197 ns | 296 ns |
| D153 | 18.2 ns | 52.4 ns | 110 ns | 232 ns | 352 ns |
| D230 | 28.4 ns | 116 ns | 220 ns | 303 ns | 994 ns |
| D307 | 44.4 ns | 184 ns | 508 ns | 1.05 µs | 967 ns |
| D462 | 94.1 ns | 304 ns | 1.32 µs | 1.67 µs | 2.62 µs |
| D616 | 90.2 ns | 536 ns | 1.77 µs | 2.01 µs | 3.91 µs |
| D924 | 130 ns | 1.5 µs | 3.02 µs | 5.09 µs | 6.06 µs |
| D1232 | 195 ns | 2.24 µs | 5.1 µs | 9 µs | 12.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.2 124.4,177.4 160.5,180.7 196.7,164.7 232.9,159.6 269.1,151.9 305.3,144.1 341.5,131.0 377.6,131.8 413.8,125.4 450.0,118.4 450.0,46.7 413.8,58.7 377.6,66.3 341.5,73.3 305.3,90.6 269.1,90.1 232.9,108.1 196.7,111.1 160.5,129.7 124.4,138.2 88.2,142.6 52.0,181.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.2 124.4,177.4 160.5,180.7 196.7,164.7 232.9,159.6 269.1,151.9 305.3,144.1 341.5,131.0 377.6,131.8 413.8,125.4 450.0,118.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.7 88.2,164.5 124.4,157.6 160.5,146.8 196.7,142.1 232.9,141.2 269.1,127.4 305.3,119.4 341.5,110.7 377.6,100.8 413.8,82.9 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,147.1 124.4,149.3 160.5,145.0 196.7,133.0 232.9,128.4 269.1,116.3 305.3,101.8 341.5,85.1 377.6,80.1 413.8,70.8 450.0,61.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.4 88.2,150.8 124.4,135.8 160.5,137.4 196.7,118.3 232.9,115.3 269.1,110.7 305.3,89.1 341.5,81.1 377.6,77.9 413.8,61.7 450.0,51.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.9 88.2,142.6 124.4,138.2 160.5,129.7 196.7,111.1 232.9,108.1 269.1,90.1 305.3,90.6 341.5,73.3 377.6,66.3 413.8,58.7 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.622 ns | 0.703 ns | 0.392 ns | 0.622 ns |
| D38 | 1.45 ns | 1.45 ns | 1.45 ns | 1.45 ns | 1.45 ns |
| D57 | 1.68 ns | 1.57 ns | 1.68 ns | 1.68 ns | 1.47 ns |
| D76 | 1.8 ns | 2.16 ns | 2.09 ns | 1.68 ns | 2.49 ns |
| D115 | 3.17 ns | 2.86 ns | 2.86 ns | 3.25 ns | 4.14 ns |
| D153 | 3.79 ns | 3.79 ns | 4.29 ns | 4.29 ns | 4.29 ns |
| D230 | 6.66 ns | 5.86 ns | 3.93 ns | 3.85 ns | 7.16 ns |
| D307 | 10.9 ns | 12.4 ns | 12.4 ns | 11 ns | 6.4 ns |
| D462 | 15 ns | 19.9 ns | 16.7 ns | 11.4 ns | 17 ns |
| D616 | 19 ns | 15.5 ns | 20.6 ns | 18 ns | 20.2 ns |
| D924 | 72 ns | 76.5 ns | 75.7 ns | 75.7 ns | 55.9 ns |
| D1232 | 54.4 ns | 70.2 ns | 69.7 ns | 70 ns | 63.3 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.7 124.4,128.2 160.5,126.4 196.7,109.9 232.9,104.8 269.1,88.4 305.3,74.2 341.5,64.9 377.6,58.1 413.8,19.5 450.0,27.6 450.0,23.2 413.8,26.8 377.6,56.4 341.5,61.3 305.3,89.6 269.1,86.3 232.9,101.1 196.7,102.2 160.5,116.9 124.4,132.2 88.2,132.6 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.7 124.4,128.2 160.5,126.4 196.7,109.9 232.9,104.8 269.1,88.4 305.3,74.2 341.5,64.9 377.6,58.1 413.8,19.5 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,132.6 124.4,130.2 160.5,121.0 196.7,112.9 232.9,104.7 269.1,92.2 305.3,70.5 341.5,56.8 377.6,64.0 413.8,17.8 450.0,20.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,128.3 160.5,122.0 196.7,112.9 232.9,101.1 269.1,103.7 305.3,70.5 341.5,61.9 377.6,55.7 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,132.6 124.4,128.3 160.5,128.3 196.7,109.2 232.9,101.1 269.1,104.3 305.3,73.9 341.5,72.9 377.6,59.7 413.8,18.1 450.0,20.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,132.2 160.5,116.9 196.7,102.2 232.9,101.1 269.1,86.3 305.3,89.6 341.5,61.3 377.6,56.4 413.8,26.8 450.0,23.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.87 ns | 2.11 ns | 3.32 ns | 2.18 ns |
| D38 | 3.52 ns | 3.51 ns | 3.51 ns | 3.52 ns | 3.52 ns |
| D57 | 7.17 ns | 6.35 ns | 7.16 ns | 7.16 ns | 4.4 ns |
| D76 | 6.63 ns | 9.83 ns | 8.48 ns | 7.63 ns | 8.71 ns |
| D115 | 14.4 ns | 12.8 ns | 12.8 ns | 12.8 ns | 16.8 ns |
| D153 | 16.5 ns | 17.1 ns | 21.8 ns | 17.7 ns | 17.8 ns |
| D230 | 36.4 ns | 31.8 ns | 16.8 ns | 18.4 ns | 31.8 ns |
| D307 | 41.8 ns | 48 ns | 48 ns | 42.9 ns | 38.4 ns |
| D462 | 74.1 ns | 56.6 ns | 86.8 ns | 53.9 ns | 82.6 ns |
| D616 | 81.6 ns | 70.2 ns | 78.7 ns | 69.1 ns | 78.1 ns |
| D924 | 132 ns | 97.1 ns | 99.8 ns | 85.3 ns | 59 ns |
| D1232 | 150 ns | 136 ns | 139 ns | 125 ns | 102 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,173.6 124.4,152.9 160.5,155.2 196.7,132.8 232.9,128.8 269.1,105.9 305.3,101.9 341.5,85.4 377.6,82.5 413.8,68.6 450.0,65.0 450.0,76.0 413.8,91.9 377.6,83.8 341.5,82.2 305.3,104.4 269.1,109.8 232.9,126.6 196.7,128.3 160.5,147.3 124.4,167.1 88.2,173.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,173.6 124.4,152.9 160.5,155.2 196.7,132.8 232.9,128.8 269.1,105.9 305.3,101.9 341.5,85.4 377.6,82.5 413.8,68.6 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,173.6 124.4,156.5 160.5,143.8 196.7,136.3 232.9,127.8 269.1,109.8 305.3,97.9 341.5,93.1 377.6,86.9 413.8,77.5 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,173.6 124.4,153.0 160.5,148.1 196.7,136.3 232.9,120.8 269.1,128.3 305.3,97.9 341.5,80.8 377.6,83.6 413.8,76.7 450.0,67.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.2 88.2,173.5 124.4,153.0 160.5,151.2 196.7,136.3 232.9,126.7 269.1,125.8 305.3,101.1 341.5,94.5 377.6,87.4 413.8,81.3 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,173.5 124.4,167.1 160.5,147.3 196.7,128.3 232.9,126.6 269.1,109.8 305.3,104.4 341.5,82.2 377.6,83.8 413.8,91.9 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.25 ns | 1.06 ns | 1.15 ns | 1.25 ns |
| D38 | 1.82 ns | 1.81 ns | 1.81 ns | 1.81 ns | 1.82 ns |
| D57 | 2.26 ns | 2.99 ns | 2.25 ns | 2.25 ns | 1.9 ns |
| D76 | 2.42 ns | 3.45 ns | 3.09 ns | 2.69 ns | 3.09 ns |
| D115 | 5.56 ns | 4.84 ns | 4.83 ns | 4.86 ns | 6.5 ns |
| D153 | 7.65 ns | 7.67 ns | 7.65 ns | 7.65 ns | 7.62 ns |
| D230 | 17.6 ns | 16.1 ns | 11.1 ns | 9.74 ns | 16.1 ns |
| D307 | 23.5 ns | 25.1 ns | 25.2 ns | 23.5 ns | 16.6 ns |
| D462 | 37.3 ns | 36.1 ns | 40.6 ns | 29.6 ns | 43.2 ns |
| D616 | 45.5 ns | 34.1 ns | 45.2 ns | 52.6 ns | 46.5 ns |
| D924 | 74.5 ns | 76.2 ns | 75.6 ns | 76.4 ns | 67.3 ns |
| D1232 | 106 ns | 105 ns | 106 ns | 106 ns | 102 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.4 88.2,192.7 124.4,186.4 160.5,184.4 196.7,160.3 232.9,151.1 269.1,126.9 305.3,118.6 341.5,105.2 377.6,99.5 413.8,85.2 450.0,74.9 450.0,76.0 413.8,88.1 377.6,98.9 341.5,101.0 305.3,128.6 269.1,129.5 232.9,151.2 196.7,155.8 160.5,177.4 124.4,191.4 88.2,192.7 52.0,203.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.4 88.2,192.7 124.4,186.4 160.5,184.4 196.7,160.3 232.9,151.1 269.1,126.9 305.3,118.6 341.5,105.2 377.6,99.5 413.8,85.2 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,203.7 88.2,192.8 124.4,178.3 160.5,174.1 196.7,164.3 232.9,151.0 269.1,129.6 305.3,116.7 341.5,106.2 377.6,107.8 413.8,84.5 450.0,75.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.4 88.2,192.9 124.4,186.5 160.5,177.3 196.7,164.4 232.9,151.1 269.1,140.4 305.3,116.6 341.5,102.7 377.6,99.6 413.8,84.8 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,205.9 88.2,192.7 124.4,186.5 160.5,181.4 196.7,164.2 232.9,151.1 269.1,144.1 305.3,118.6 341.5,111.9 377.6,95.3 413.8,84.5 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,203.6 88.2,192.7 124.4,191.4 160.5,177.4 196.7,155.8 232.9,151.2 269.1,129.5 305.3,128.6 341.5,101.0 377.6,98.9 413.8,88.1 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
