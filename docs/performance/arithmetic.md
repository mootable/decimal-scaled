# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.84 ns | 0.981 ns | 1.15 ns | 0.936 ns |
| D38 | 1.41 ns | 1.62 ns | 1.62 ns | 1.61 ns | 1.64 ns |
| D57 | 2.5 ns | 2.5 ns | 2.28 ns | 2.28 ns | 1.36 ns |
| D76 | 2.18 ns | 3.48 ns | 3.08 ns | 2.15 ns | 3.1 ns |
| D115 | 5.01 ns | 4.39 ns | 4.98 ns | 4.99 ns | 4.99 ns |
| D153 | 6.67 ns | 5.89 ns | 5.91 ns | 6.62 ns | 6.63 ns |
| D230 | 15.3 ns | 11.9 ns | 10 ns | 13.9 ns | 13.9 ns |
| D307 | 18.6 ns | 14.6 ns | 19.6 ns | 14.6 ns | 18.6 ns |
| D462 | 37.4 ns | 21.8 ns | 32.6 ns | 29.2 ns | 42.9 ns |
| D616 | 60.9 ns | 69.8 ns | 45.3 ns | 49.3 ns | 51.1 ns |
| D924 | 55.5 ns | 74.8 ns | 85 ns | 55.5 ns | 84.8 ns |
| D1232 | 84 ns | 95 ns | 104 ns | 94.9 ns | 104 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,152.5 124.4,140.1 160.5,143.1 196.7,125.0 232.9,118.8 269.1,100.8 305.3,96.6 341.5,81.3 377.6,70.8 413.8,72.8 450.0,63.8 450.0,59.1 413.8,63.6 377.6,74.6 341.5,78.4 305.3,96.6 269.1,102.9 232.9,118.9 196.7,125.1 160.5,135.5 124.4,153.3 88.2,149.2 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,152.5 124.4,140.1 160.5,143.1 196.7,125.0 232.9,118.8 269.1,100.8 305.3,96.6 341.5,81.3 377.6,70.8 413.8,72.8 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.8 88.2,149.6 124.4,140.1 160.5,132.9 196.7,127.9 232.9,121.5 269.1,106.2 305.3,101.8 341.5,93.1 377.6,67.8 413.8,66.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.4 88.2,149.6 124.4,142.1 160.5,135.5 196.7,125.1 232.9,121.4 269.1,110.0 305.3,95.4 341.5,84.3 377.6,77.2 413.8,63.5 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,149.6 124.4,142.1 160.5,143.3 196.7,125.1 232.9,118.9 269.1,102.9 305.3,101.8 341.5,86.7 377.6,75.4 413.8,72.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.2 124.4,153.3 160.5,135.5 196.7,125.1 232.9,118.9 269.1,102.9 305.3,96.6 341.5,78.4 377.6,74.6 413.8,63.6 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.49 ns | 4 ns | 4.63 ns | 8.5 ns | 7.86 ns |
| D38 | 9.85 ns | 11.6 ns | 15.6 ns | 59 ns | 60.1 ns |
| D57 | 35.4 ns | 51.3 ns | 68.7 ns | 107 ns | 71.7 ns |
| D76 | 32.4 ns | 66.7 ns | 76 ns | 100 ns | 133 ns |
| D115 | 58.3 ns | 83.1 ns | 111 ns | 193 ns | 240 ns |
| D153 | 77 ns | 108 ns | 144 ns | 251 ns | 334 ns |
| D230 | 111 ns | 130 ns | 149 ns | 384 ns | 562 ns |
| D307 | 144 ns | 196 ns | 399 ns | 600 ns | 882 ns |
| D462 | 205 ns | 236 ns | 718 ns | 1.06 µs | 1.48 µs |
| D616 | 261 ns | 657 ns | 1.05 µs | 1.82 µs | 2.39 µs |
| D924 | 306 ns | 1.13 µs | 2.28 µs | 2.7 µs | 4.69 µs |
| D1232 | 486 ns | 1.75 µs | 3.5 µs | 3.69 µs | 7.41 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,160.3 124.4,132.5 160.5,134.5 196.7,121.7 232.9,115.7 269.1,107.8 305.3,102.1 341.5,94.4 377.6,89.1 413.8,85.7 450.0,75.7 450.0,16.5 413.8,26.4 377.6,41.1 341.5,51.5 305.3,62.7 269.1,72.5 232.9,83.8 196.7,91.0 160.5,103.9 124.4,117.2 88.2,121.1 52.0,165.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,160.3 124.4,132.5 160.5,134.5 196.7,121.7 232.9,115.7 269.1,107.8 305.3,102.1 341.5,94.4 377.6,89.1 413.8,85.7 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.9 88.2,156.7 124.4,124.5 160.5,118.8 196.7,114.0 232.9,108.3 269.1,104.4 305.3,95.4 341.5,91.3 377.6,69.1 413.8,57.3 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,176.7 88.2,150.4 124.4,118.2 160.5,116.0 196.7,107.8 232.9,102.1 269.1,101.4 305.3,79.9 341.5,67.2 377.6,59.0 413.8,42.1 450.0,32.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.5 88.2,121.5 124.4,108.5 160.5,110.0 196.7,95.7 232.9,90.1 269.1,80.8 305.3,71.1 341.5,58.7 377.6,47.0 413.8,38.5 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.2 88.2,121.1 124.4,117.2 160.5,103.9 196.7,91.0 232.9,83.8 269.1,72.5 305.3,62.7 341.5,51.5 377.6,41.1 413.8,26.4 450.0,16.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.74 ns | 2.21 ns | 4.23 ns | 5.12 ns |
| D38 | 3.07 ns | 13.8 ns | 26.3 ns | 27.4 ns | 27.7 ns |
| D57 | 4.23 ns | 22 ns | 33.4 ns | 71.7 ns | 46 ns |
| D76 | 5.41 ns | 38.1 ns | 42.4 ns | 65.5 ns | 101 ns |
| D115 | 13.6 ns | 47.5 ns | 93.2 ns | 215 ns | 253 ns |
| D153 | 16.9 ns | 50.1 ns | 112 ns | 258 ns | 396 ns |
| D230 | 27.7 ns | 98.9 ns | 215 ns | 520 ns | 995 ns |
| D307 | 44.8 ns | 146 ns | 510 ns | 933 ns | 1.37 µs |
| D462 | 67 ns | 238 ns | 1.29 µs | 1.75 µs | 2.61 µs |
| D616 | 102 ns | 723 ns | 1.72 µs | 2.68 µs | 4.15 µs |
| D924 | 113 ns | 1.47 µs | 3.16 µs | 4.95 µs | 8.19 µs |
| D1232 | 155 ns | 2.19 µs | 4.59 µs | 6.9 µs | 12.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,190.5 124.4,185.0 160.5,180.7 196.7,164.7 232.9,160.9 269.1,152.3 305.3,143.9 341.5,136.9 377.6,129.6 413.8,127.9 450.0,122.3 450.0,45.6 413.8,53.5 377.6,65.3 341.5,73.4 305.3,84.5 269.1,90.1 232.9,106.1 196.7,113.9 160.5,129.8 124.4,143.5 88.2,152.3 52.0,181.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,190.5 124.4,185.0 160.5,180.7 196.7,164.7 232.9,160.9 269.1,152.3 305.3,143.9 341.5,136.9 377.6,129.6 413.8,127.9 450.0,122.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.3 88.2,164.4 124.4,156.3 160.5,146.8 196.7,142.9 232.9,142.0 269.1,130.2 305.3,123.5 341.5,114.9 377.6,95.6 413.8,83.3 450.0,76.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.2 88.2,153.2 124.4,149.1 160.5,144.9 196.7,131.2 232.9,128.1 269.1,116.7 305.3,101.7 341.5,85.5 377.6,80.6 413.8,70.0 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.9 88.2,152.5 124.4,135.8 160.5,137.4 196.7,116.7 232.9,113.5 269.1,101.4 305.3,91.2 341.5,80.3 377.6,72.9 413.8,62.2 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.6 88.2,152.3 124.4,143.5 160.5,129.8 196.7,113.9 232.9,106.1 269.1,90.1 305.3,84.5 341.5,73.4 377.6,65.3 413.8,53.5 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.27 ns | 0.317 ns | 0.369 ns | 0.622 ns |
| D38 | 1.12 ns | 1.33 ns | 1.32 ns | 1.32 ns | 1.32 ns |
| D57 | 1.74 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.05 ns |
| D76 | 1.8 ns | 2.16 ns | 2.18 ns | 1.76 ns | 2.49 ns |
| D115 | 3.17 ns | 2.85 ns | 3.17 ns | 3.55 ns | 3.55 ns |
| D153 | 4.22 ns | 3.79 ns | 4.49 ns | 4.6 ns | 4.6 ns |
| D230 | 6.65 ns | 5.16 ns | 3.91 ns | 7.16 ns | 7.16 ns |
| D307 | 10.9 ns | 7.68 ns | 12.5 ns | 7.77 ns | 11.1 ns |
| D462 | 14 ns | 11.1 ns | 16.7 ns | 15.3 ns | 17 ns |
| D616 | 23.7 ns | 32.5 ns | 20.1 ns | 20 ns | 21.9 ns |
| D924 | 39.4 ns | 75.1 ns | 84.6 ns | 59.3 ns | 84.8 ns |
| D1232 | 41.7 ns | 61.4 ns | 64.8 ns | 72.7 ns | 65.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,140.0 124.4,127.3 160.5,126.3 196.7,109.9 232.9,101.6 269.1,88.5 305.3,74.2 341.5,67.0 377.6,51.7 413.8,36.9 450.0,35.3 450.0,22.2 413.8,14.8 377.6,53.9 341.5,61.3 305.3,73.7 269.1,86.3 232.9,99.1 196.7,106.6 160.5,116.9 124.4,142.0 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,140.0 124.4,127.3 160.5,126.3 196.7,109.9 232.9,101.6 269.1,88.5 305.3,74.2 341.5,67.0 377.6,51.7 413.8,36.9 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,135.1 124.4,127.2 160.5,121.1 196.7,113.0 232.9,104.8 269.1,95.8 305.3,84.3 341.5,73.7 377.6,42.6 413.8,18.3 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,176.6 88.2,135.2 124.4,125.2 160.5,120.7 196.7,110.0 232.9,99.9 269.1,103.9 305.3,70.2 341.5,61.8 377.6,56.5 413.8,14.8 450.0,22.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,135.2 124.4,125.2 160.5,126.9 196.7,106.7 232.9,99.1 269.1,86.3 305.3,84.0 341.5,64.3 377.6,56.7 413.8,25.2 450.0,19.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,142.0 160.5,116.9 196.7,106.6 232.9,99.1 269.1,86.3 305.3,73.7 341.5,61.3 377.6,53.9 413.8,14.8 450.0,22.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.44 ns | 1.7 ns | 3.25 ns | 2.18 ns |
| D38 | 6.34 ns | 7.14 ns | 12.1 ns | 12.5 ns | 13.1 ns |
| D57 | 8.08 ns | 8.09 ns | 7.17 ns | 7.16 ns | 3.44 ns |
| D76 | 6.4 ns | 9.5 ns | 8.56 ns | 5.88 ns | 8.71 ns |
| D115 | 14.4 ns | 12.4 ns | 14.1 ns | 14.1 ns | 14.1 ns |
| D153 | 20.8 ns | 15.9 ns | 15.9 ns | 20 ns | 20 ns |
| D230 | 36.9 ns | 28.2 ns | 16.8 ns | 32.1 ns | 32.1 ns |
| D307 | 41.9 ns | 30.4 ns | 47.9 ns | 28.8 ns | 42.5 ns |
| D462 | 74.1 ns | 40.1 ns | 89.9 ns | 79 ns | 88 ns |
| D616 | 98.7 ns | 112 ns | 82.8 ns | 81.9 ns | 107 ns |
| D924 | 74.2 ns | 109 ns | 108 ns | 62 ns | 109 ns |
| D1232 | 133 ns | 127 ns | 129 ns | 107 ns | 118 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,156.5 124.4,149.5 160.5,156.2 196.7,132.7 232.9,122.1 269.1,105.6 305.3,101.8 341.5,85.4 377.6,77.0 413.8,85.3 450.0,68.5 450.0,71.9 413.8,74.2 377.6,74.8 341.5,80.4 305.3,101.4 269.1,109.6 232.9,123.2 196.7,133.5 160.5,147.3 124.4,174.2 88.2,135.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,156.5 124.4,149.5 160.5,156.2 196.7,132.7 232.9,122.1 269.1,105.6 305.3,101.8 341.5,85.4 377.6,77.0 413.8,85.3 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.4 88.2,153.1 124.4,149.5 160.5,144.8 196.7,137.1 232.9,129.8 269.1,113.4 305.3,111.2 341.5,103.1 377.6,73.4 413.8,74.1 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.7 88.2,137.9 124.4,153.0 160.5,147.8 196.7,133.5 232.9,129.9 269.1,128.3 305.3,98.0 341.5,79.8 377.6,82.1 413.8,74.4 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.9 88.2,136.8 124.4,153.0 160.5,158.7 196.7,133.5 232.9,123.2 269.1,109.6 305.3,112.7 341.5,83.5 377.6,82.4 413.8,90.5 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,135.5 124.4,174.2 160.5,147.3 196.7,133.5 232.9,123.2 269.1,109.6 305.3,101.4 341.5,80.4 377.6,74.8 413.8,74.2 450.0,71.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.93 ns | 0.981 ns | 1.14 ns | 0.936 ns |
| D38 | 1.41 ns | 1.6 ns | 1.6 ns | 1.61 ns | 1.62 ns |
| D57 | 2.5 ns | 2.51 ns | 2.27 ns | 2.28 ns | 1.3 ns |
| D76 | 2.6 ns | 3.46 ns | 3.1 ns | 2.52 ns | 3.1 ns |
| D115 | 5.54 ns | 4.83 ns | 5.53 ns | 5.54 ns | 5.56 ns |
| D153 | 8.46 ns | 7.68 ns | 7.64 ns | 8.48 ns | 8.41 ns |
| D230 | 17.6 ns | 13.7 ns | 11.2 ns | 16.1 ns | 16.2 ns |
| D307 | 23.3 ns | 18 ns | 25.1 ns | 18.1 ns | 23.4 ns |
| D462 | 43.5 ns | 26.4 ns | 42.4 ns | 37.3 ns | 52.8 ns |
| D616 | 62.6 ns | 76.5 ns | 45.9 ns | 50.1 ns | 49 ns |
| D924 | 61.2 ns | 74.9 ns | 84.9 ns | 60.8 ns | 84.9 ns |
| D1232 | 83.7 ns | 95.5 ns | 105 ns | 93.8 ns | 105 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,152.6 124.4,140.1 160.5,139.3 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.6 341.5,78.1 377.6,70.2 413.8,70.7 450.0,63.9 450.0,59.0 413.8,63.5 377.6,75.5 341.5,73.9 305.3,91.6 269.1,99.5 232.9,113.8 196.7,122.8 160.5,135.4 124.4,154.3 88.2,149.5 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,152.6 124.4,140.1 160.5,139.3 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.6 341.5,78.1 377.6,70.2 413.8,70.7 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.6 88.2,149.7 124.4,140.0 160.5,133.1 196.7,125.8 232.9,115.7 269.1,103.2 305.3,97.2 341.5,88.9 377.6,65.8 413.8,66.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.4 88.2,149.8 124.4,142.2 160.5,135.5 196.7,122.9 232.9,115.8 269.1,107.6 305.3,90.0 341.5,78.6 377.6,76.9 413.8,63.6 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,149.6 124.4,142.1 160.5,139.9 196.7,122.8 232.9,113.6 269.1,99.6 305.3,97.2 341.5,81.4 377.6,75.0 413.8,70.8 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.5 124.4,154.3 160.5,135.4 196.7,122.8 232.9,113.8 269.1,99.5 305.3,91.6 341.5,73.9 377.6,75.5 413.8,63.5 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
