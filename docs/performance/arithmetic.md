# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.937 ns | 0.936 ns | 0.934 ns | 0.852 ns | 0.822 ns |
| D38 | 1.04 ns | 1.62 ns | 1.64 ns | 1.4 ns | 1.11 ns |
| D57 | 1.53 ns | 2.28 ns | 2.28 ns | 2.28 ns | 2.28 ns |
| D76 | 3.48 ns | 1.56 ns | 1.54 ns | 3.08 ns | 3.49 ns |
| D115 | 4.4 ns | 3.54 ns | 4.4 ns | 3.88 ns | 4.41 ns |
| D153 | 5.91 ns | 5.15 ns | 6.63 ns | 3.78 ns | 4.59 ns |
| D230 | 11.7 ns | 13.9 ns | 13.8 ns | 13.9 ns | 13 ns |
| D307 | 13.5 ns | 13.8 ns | 19.6 ns | 18.6 ns | 15.2 ns |
| D462 | 26.2 ns | 28.9 ns | 28.9 ns | 51.5 ns | 32.7 ns |
| D616 | 45.2 ns | 45.3 ns | 50.2 ns | 33.5 ns | 50 ns |
| D924 | 55.2 ns | 83.4 ns | 55.2 ns | 71.9 ns | 73.5 ns |
| D1232 | 95.1 ns | 121 ns | 70.8 ns | 95.3 ns | 120 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,159.2 124.4,150.8 160.5,132.9 196.7,127.8 232.9,121.4 269.1,106.5 305.3,103.4 341.5,89.1 377.6,77.3 413.8,72.9 450.0,61.1 450.0,56.0 413.8,66.7 377.6,75.0 341.5,84.3 305.3,100.9 269.1,104.2 232.9,126.9 196.7,127.8 160.5,132.9 124.4,142.1 88.2,157.8 52.0,164.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,159.2 124.4,150.8 160.5,132.9 196.7,127.8 232.9,121.4 269.1,106.5 305.3,103.4 341.5,89.1 377.6,77.3 413.8,72.9 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.1 160.5,150.4 196.7,132.5 232.9,124.4 269.1,102.8 305.3,103.0 341.5,87.0 377.6,77.2 413.8,64.0 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.3 124.4,142.1 160.5,150.7 196.7,127.8 232.9,118.9 269.1,102.9 305.3,95.4 341.5,87.0 377.6,75.0 413.8,72.9 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.5 88.2,152.7 124.4,142.1 160.5,135.6 196.7,130.6 232.9,131.1 269.1,102.9 305.3,96.6 341.5,74.4 377.6,83.7 413.8,67.2 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,157.8 124.4,142.1 160.5,132.9 196.7,127.8 232.9,126.9 269.1,104.2 305.3,100.9 341.5,84.3 377.6,75.0 413.8,66.7 450.0,56.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.08 ns | 5.91 ns | 5.51 ns | 6.02 ns | 6.92 ns |
| D38 | 7.58 ns | 11.4 ns | 15.2 ns | 53.8 ns | 48.2 ns |
| D57 | 19.6 ns | 33.5 ns | 68.1 ns | 111 ns | 112 ns |
| D76 | 25.3 ns | 34.7 ns | 46 ns | 115 ns | 149 ns |
| D115 | 43.4 ns | 66.5 ns | 104 ns | 156 ns | 229 ns |
| D153 | 55 ns | 89.4 ns | 155 ns | 179 ns | 293 ns |
| D230 | 47.4 ns | 154 ns | 231 ns | 392 ns | 533 ns |
| D307 | 70.9 ns | 152 ns | 398 ns | 598 ns | 763 ns |
| D462 | 192 ns | 421 ns | 687 ns | 1.17 µs | 1.49 µs |
| D616 | 237 ns | 627 ns | 904 ns | 1.71 µs | 1.9 µs |
| D924 | 282 ns | 1.12 µs | 1.86 µs | 2.39 µs | 3.69 µs |
| D1232 | 508 ns | 2 µs | 3.02 µs | 4.12 µs | 7.82 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.8 88.2,166.0 124.4,145.4 160.5,139.8 196.7,128.1 232.9,123.0 269.1,126.2 305.3,117.5 341.5,95.8 377.6,91.3 413.8,87.5 450.0,74.7 450.0,15.3 413.8,31.6 377.6,46.0 341.5,51.4 305.3,65.9 269.1,73.6 232.9,86.7 196.7,92.0 160.5,101.3 124.4,107.5 88.2,125.9 52.0,168.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.8 88.2,166.0 124.4,145.4 160.5,139.8 196.7,128.1 232.9,123.0 269.1,126.2 305.3,117.5 341.5,95.8 377.6,91.3 413.8,87.5 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,157.1 124.4,133.7 160.5,133.0 196.7,118.9 232.9,112.4 269.1,100.6 305.3,100.9 341.5,78.8 377.6,70.2 413.8,57.5 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.0 88.2,151.0 124.4,118.3 160.5,126.9 196.7,109.1 232.9,100.5 269.1,91.8 305.3,80.0 341.5,68.1 377.6,62.2 413.8,46.6 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.0 88.2,123.5 124.4,107.7 160.5,106.9 196.7,100.3 232.9,97.3 269.1,80.3 305.3,71.2 341.5,56.6 377.6,48.3 413.8,41.1 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.0 88.2,125.9 124.4,107.5 160.5,101.3 196.7,92.0 232.9,86.7 269.1,73.6 305.3,65.9 341.5,51.4 377.6,46.0 413.8,31.6 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.881 ns | 2.99 ns | 3.13 ns | 3.33 ns | 3.84 ns |
| D38 | 2.62 ns | 14.3 ns | 26.2 ns | 23.8 ns | 15.2 ns |
| D57 | 3.46 ns | 21.3 ns | 32.8 ns | 72.5 ns | 71.3 ns |
| D76 | 5.65 ns | 22.5 ns | 23.4 ns | 79.3 ns | 108 ns |
| D115 | 13.5 ns | 37.2 ns | 87 ns | 165 ns | 229 ns |
| D153 | 18.1 ns | 43 ns | 118 ns | 181 ns | 327 ns |
| D230 | 18.1 ns | 116 ns | 345 ns | 527 ns | 936 ns |
| D307 | 26 ns | 113 ns | 508 ns | 1.02 µs | 1.15 µs |
| D462 | 71.1 ns | 412 ns | 1.25 µs | 1.87 µs | 2.64 µs |
| D616 | 95.1 ns | 921 ns | 1.48 µs | 2.63 µs | 3.34 µs |
| D924 | 113 ns | 1.48 µs | 2.91 µs | 4.28 µs | 6.05 µs |
| D1232 | 192 ns | 2.39 µs | 4.55 µs | 8.28 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.5 88.2,162.7 124.4,158.7 160.5,151.6 196.7,139.0 232.9,134.8 269.1,134.7 305.3,129.5 341.5,114.9 377.6,110.7 413.8,108.2 450.0,100.6 450.0,38.3 413.8,50.6 377.6,59.2 341.5,62.6 305.3,74.7 269.1,77.6 232.9,92.8 196.7,98.0 160.5,108.8 124.4,114.9 88.2,137.3 52.0,157.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.5 88.2,162.7 124.4,158.7 160.5,151.6 196.7,139.0 232.9,134.8 269.1,134.7 305.3,129.5 341.5,114.9 377.6,110.7 413.8,108.2 450.0,100.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.8 88.2,138.2 124.4,132.4 160.5,131.6 196.7,124.3 232.9,122.2 269.1,107.8 305.3,108.2 341.5,89.5 377.6,77.9 413.8,71.0 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,129.4 124.4,126.1 160.5,131.0 196.7,112.0 232.9,107.6 269.1,92.1 305.3,86.5 341.5,73.4 377.6,71.0 413.8,61.2 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.2 88.2,130.8 124.4,114.7 160.5,113.4 196.7,102.7 232.9,101.4 269.1,85.9 305.3,76.3 341.5,67.6 377.6,62.6 413.8,55.6 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.2 88.2,137.3 124.4,114.9 160.5,108.8 196.7,98.0 232.9,92.8 269.1,77.6 305.3,74.7 341.5,62.6 377.6,59.2 413.8,50.6 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.622 ns | 0.622 ns | 0.278 ns | 0.546 ns |
| D38 | 0.972 ns | 1.32 ns | 1.33 ns | 1.12 ns | 0.803 ns |
| D57 | 1.3 ns | 1.68 ns | 1.68 ns | 1.68 ns | 1.68 ns |
| D76 | 2.17 ns | 1.33 ns | 1.11 ns | 2.09 ns | 2.63 ns |
| D115 | 2.86 ns | 2.82 ns | 2.87 ns | 2.75 ns | 3.25 ns |
| D153 | 3.82 ns | 3.27 ns | 4.6 ns | 2.8 ns | 3.39 ns |
| D230 | 4.18 ns | 5.86 ns | 7.16 ns | 7.16 ns | 7.15 ns |
| D307 | 5.6 ns | 6.59 ns | 12.4 ns | 11 ns | 9.58 ns |
| D462 | 14.2 ns | 15.1 ns | 15.2 ns | 16.6 ns | 16.7 ns |
| D616 | 19 ns | 20.1 ns | 18.1 ns | 15 ns | 17.2 ns |
| D924 | 38.2 ns | 82.4 ns | 59.5 ns | 61.7 ns | 67.2 ns |
| D1232 | 47.2 ns | 71.5 ns | 44.5 ns | 61.8 ns | 71.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,144.2 124.4,135.6 160.5,121.0 196.7,112.9 232.9,104.5 269.1,101.9 305.3,93.5 341.5,66.5 377.6,58.0 413.8,37.9 450.0,31.7 450.0,19.6 413.8,21.5 377.6,61.0 341.5,61.9 305.3,77.9 269.1,86.4 232.9,108.0 196.7,109.2 160.5,115.4 124.4,128.3 88.2,149.7 52.0,160.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,144.2 124.4,135.6 160.5,121.0 196.7,112.9 232.9,104.5 269.1,101.9 305.3,93.5 341.5,66.5 377.6,58.0 413.8,37.9 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.2 124.4,128.2 160.5,135.1 196.7,113.3 232.9,109.0 269.1,92.1 305.3,88.7 341.5,64.8 377.6,56.4 413.8,15.6 450.0,19.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,140.3 196.7,112.9 232.9,99.2 269.1,86.3 305.3,70.5 341.5,64.6 377.6,59.6 413.8,25.0 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.4 88.2,140.0 124.4,128.3 160.5,121.9 196.7,114.0 232.9,113.5 269.1,86.3 305.3,73.9 341.5,61.9 377.6,64.9 413.8,24.0 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,149.7 124.4,128.3 160.5,115.4 196.7,109.2 232.9,108.0 269.1,86.4 305.3,77.9 341.5,61.9 377.6,61.0 413.8,21.5 450.0,19.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 1.87 ns | 2.4 ns | 1.91 ns |
| D38 | 2.24 ns | 3.24 ns | 3.26 ns | 3 ns | 1.76 ns |
| D57 | 4.55 ns | 7.17 ns | 7.17 ns | 7.16 ns | 7.16 ns |
| D76 | 9.84 ns | 4.69 ns | 4.12 ns | 8.71 ns | 9.83 ns |
| D115 | 12.9 ns | 9.73 ns | 12.7 ns | 11.2 ns | 12.7 ns |
| D153 | 16.1 ns | 15.6 ns | 20 ns | 9.89 ns | 12.9 ns |
| D230 | 19.6 ns | 32.4 ns | 31.9 ns | 32.1 ns | 31.6 ns |
| D307 | 27.4 ns | 27.6 ns | 48 ns | 42.7 ns | 37.1 ns |
| D462 | 66.9 ns | 74.1 ns | 74.2 ns | 97.1 ns | 83.1 ns |
| D616 | 85.1 ns | 82.2 ns | 74.8 ns | 61.4 ns | 70.1 ns |
| D924 | 67.4 ns | 102 ns | 58.1 ns | 79.5 ns | 85.2 ns |
| D1232 | 132 ns | 140 ns | 86 ns | 126 ns | 130 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,186.7 124.4,166.1 160.5,143.8 196.7,135.9 232.9,129.5 269.1,123.8 305.3,114.1 341.5,88.3 377.6,81.3 413.8,88.1 450.0,68.6 450.0,69.1 413.8,81.3 377.6,86.9 341.5,82.0 305.3,105.4 269.1,110.0 232.9,135.9 196.7,136.5 160.5,143.8 124.4,153.0 88.2,193.6 52.0,191.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,186.7 124.4,166.1 160.5,143.8 196.7,135.9 232.9,129.5 269.1,123.8 305.3,114.1 341.5,88.3 377.6,81.3 413.8,88.1 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,176.0 124.4,153.0 160.5,165.2 196.7,144.1 232.9,130.5 269.1,109.3 305.3,113.9 341.5,85.3 377.6,82.3 413.8,76.2 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,175.8 124.4,153.0 160.5,169.0 196.7,136.3 232.9,123.2 269.1,109.8 305.3,97.9 341.5,85.3 377.6,85.1 413.8,92.4 450.0,81.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.6 88.2,178.2 124.4,153.0 160.5,147.3 196.7,140.1 232.9,143.6 269.1,109.6 305.3,101.3 341.5,77.5 377.6,90.8 413.8,83.3 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.3 88.2,193.6 124.4,153.0 160.5,143.8 196.7,136.5 232.9,135.9 269.1,110.0 305.3,105.4 341.5,82.0 377.6,86.9 413.8,81.3 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 0.938 ns | 0.935 ns | 0.841 ns | 0.82 ns |
| D38 | 0.95 ns | 1.61 ns | 1.6 ns | 1.41 ns | 0.778 ns |
| D57 | 1.71 ns | 2.27 ns | 2.25 ns | 2.26 ns | 2.25 ns |
| D76 | 3.48 ns | 1.82 ns | 1.6 ns | 3.09 ns | 3.45 ns |
| D115 | 4.83 ns | 4.47 ns | 4.89 ns | 4.33 ns | 4.84 ns |
| D153 | 7.68 ns | 6.6 ns | 8.42 ns | 5.17 ns | 6.26 ns |
| D230 | 12.9 ns | 16.2 ns | 16.2 ns | 16.2 ns | 16.1 ns |
| D307 | 15.7 ns | 16.2 ns | 25.2 ns | 23.1 ns | 19.5 ns |
| D462 | 33.1 ns | 37 ns | 37.2 ns | 59.2 ns | 42.5 ns |
| D616 | 45.8 ns | 45.6 ns | 50.3 ns | 35.4 ns | 50 ns |
| D924 | 60.9 ns | 84.1 ns | 60.9 ns | 78.1 ns | 79.9 ns |
| D1232 | 95.1 ns | 121 ns | 77.7 ns | 96.4 ns | 119 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,161.1 124.4,148.4 160.5,132.9 196.7,125.8 232.9,115.7 269.1,104.5 305.3,100.2 341.5,84.0 377.6,77.0 413.8,70.8 450.0,61.1 450.0,56.2 413.8,64.9 377.6,75.0 341.5,78.6 305.3,95.5 269.1,99.6 232.9,120.2 196.7,125.7 160.5,133.1 124.4,142.4 88.2,165.5 52.0,164.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,161.1 124.4,148.4 160.5,132.9 196.7,125.8 232.9,115.7 269.1,104.5 305.3,100.2 341.5,84.0 377.6,77.0 413.8,70.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,149.7 124.4,142.2 160.5,147.0 196.7,127.5 232.9,119.0 269.1,99.6 305.3,99.5 341.5,81.6 377.6,77.0 413.8,63.7 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.7 124.4,142.3 160.5,149.8 196.7,125.5 232.9,113.7 269.1,99.6 305.3,89.9 341.5,81.4 377.6,74.9 413.8,70.8 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.8 88.2,152.5 124.4,142.3 160.5,135.5 196.7,128.2 232.9,124.3 269.1,99.6 305.3,91.9 341.5,71.4 377.6,82.5 413.8,65.4 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,165.5 124.4,142.4 160.5,133.1 196.7,125.7 232.9,120.2 269.1,99.6 305.3,95.5 341.5,78.6 377.6,75.0 413.8,64.9 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
