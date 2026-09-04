# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.25 ns | 1.06 ns | 1.25 ns | 0.819 ns |
| D38 | 1.41 ns | 1.62 ns | 1.83 ns | 1.62 ns | 1.45 ns |
| D57 | 2.28 ns | 2.29 ns | 2.51 ns | 2.5 ns | 1.73 ns |
| D76 | 3.08 ns | 3.48 ns | 3.09 ns | 3.09 ns | 2.12 ns |
| D115 | 3.54 ns | 4.4 ns | 4.4 ns | 4.4 ns | 4.4 ns |
| D153 | 6.63 ns | 5.15 ns | 3.86 ns | 4.66 ns | 6.63 ns |
| D230 | 11.9 ns | 13.9 ns | 15.3 ns | 10.1 ns | 11.9 ns |
| D307 | 19.6 ns | 15.2 ns | 12.1 ns | 19.7 ns | 16.3 ns |
| D462 | 29.5 ns | 27.2 ns | 29.4 ns | 27.2 ns | 29.3 ns |
| D616 | 44.9 ns | 41.2 ns | 48.9 ns | 52.4 ns | 56.8 ns |
| D924 | 83.2 ns | 62.2 ns | 61.6 ns | 80 ns | 81 ns |
| D1232 | 95.3 ns | 67.1 ns | 107 ns | 93.6 ns | 95 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,152.5 124.4,142.1 160.5,135.6 196.7,132.6 232.9,118.9 269.1,106.3 305.3,95.4 341.5,86.5 377.6,77.4 413.8,64.0 450.0,61.1 450.0,61.1 413.8,64.6 377.6,72.3 341.5,86.6 305.3,99.4 269.1,106.3 232.9,118.9 196.7,127.8 160.5,143.6 124.4,148.1 88.2,152.0 52.0,164.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,152.5 124.4,142.1 160.5,135.6 196.7,132.6 232.9,118.9 269.1,106.3 305.3,95.4 341.5,86.5 377.6,77.4 413.8,64.0 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.2 88.2,149.5 124.4,142.0 160.5,133.0 196.7,127.8 232.9,124.4 269.1,102.9 305.3,100.9 341.5,88.2 377.6,79.2 413.8,70.3 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,146.9 124.4,140.0 160.5,135.5 196.7,127.8 232.9,130.6 269.1,100.8 305.3,105.9 341.5,86.6 377.6,75.5 413.8,70.5 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,149.5 124.4,140.1 160.5,135.5 196.7,127.8 232.9,126.6 269.1,109.7 305.3,95.3 341.5,88.3 377.6,74.0 413.8,64.8 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,152.0 124.4,148.1 160.5,143.6 196.7,127.8 232.9,118.9 269.1,106.3 305.3,99.4 341.5,86.6 377.6,72.3 413.8,64.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.45 ns | 5.91 ns | 6.2 ns | 7.63 ns | 7 ns |
| D38 | 9.8 ns | 11.6 ns | 14.3 ns | 60.5 ns | 65.7 ns |
| D57 | 23.4 ns | 33.1 ns | 74.7 ns | 119 ns | 99.9 ns |
| D76 | 26.3 ns | 65.4 ns | 75.7 ns | 115 ns | 134 ns |
| D115 | 32.2 ns | 85.1 ns | 105 ns | 188 ns | 231 ns |
| D153 | 58 ns | 89.5 ns | 106 ns | 216 ns | 353 ns |
| D230 | 71.9 ns | 155 ns | 254 ns | 264 ns | 486 ns |
| D307 | 133 ns | 192 ns | 229 ns | 642 ns | 873 ns |
| D462 | 190 ns | 388 ns | 688 ns | 929 ns | 1.34 µs |
| D616 | 234 ns | 469 ns | 1.04 µs | 1.98 µs | 2.4 µs |
| D924 | 367 ns | 706 ns | 1.69 µs | 2.25 µs | 3.68 µs |
| D1232 | 506 ns | 1.13 µs | 3.82 µs | 3.66 µs | 7.19 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.5 88.2,160.4 124.4,141.5 160.5,139.0 196.7,134.6 232.9,121.8 269.1,117.2 305.3,103.8 341.5,96.0 377.6,91.5 413.8,81.8 450.0,74.8 450.0,17.2 413.8,31.7 377.6,41.0 341.5,53.6 305.3,62.9 269.1,75.7 232.9,82.6 196.7,91.9 160.5,103.6 124.4,110.0 88.2,119.1 52.0,167.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.5 88.2,160.4 124.4,141.5 160.5,139.0 196.7,134.6 232.9,121.8 269.1,117.2 305.3,103.8 341.5,96.0 377.6,91.5 413.8,81.8 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,156.9 124.4,134.0 160.5,119.2 196.7,113.5 232.9,112.4 269.1,100.5 305.3,95.8 341.5,80.6 377.6,76.5 413.8,67.5 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,152.2 124.4,116.3 160.5,116.1 196.7,108.9 232.9,108.7 269.1,89.8 305.3,92.0 341.5,68.1 377.6,59.2 413.8,48.6 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.9 88.2,120.9 124.4,106.2 160.5,106.9 196.7,96.3 232.9,93.2 269.1,89.0 305.3,69.6 341.5,61.6 377.6,45.1 413.8,42.4 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.8 88.2,119.1 124.4,110.0 160.5,103.6 196.7,91.9 232.9,82.6 269.1,75.7 305.3,62.9 341.5,53.6 377.6,41.0 413.8,31.7 450.0,17.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3 ns | 3.2 ns | 4.92 ns | 3.86 ns |
| D38 | 3.06 ns | 14.4 ns | 27.8 ns | 26.2 ns | 25 ns |
| D57 | 6.54 ns | 21.3 ns | 34.5 ns | 76.2 ns | 61.4 ns |
| D76 | 7.8 ns | 38.1 ns | 42.3 ns | 79.2 ns | 83.7 ns |
| D115 | 12.8 ns | 50.5 ns | 86.4 ns | 197 ns | 229 ns |
| D153 | 16.9 ns | 42.7 ns | 79.7 ns | 217 ns | 396 ns |
| D230 | 21.7 ns | 116 ns | 368 ns | 349 ns | 848 ns |
| D307 | 54.4 ns | 143 ns | 295 ns | 1.07 µs | 1.19 µs |
| D462 | 102 ns | 366 ns | 1.29 µs | 1.48 µs | 2.45 µs |
| D616 | 91.4 ns | 509 ns | 1.78 µs | 2.9 µs | 4.18 µs |
| D924 | 146 ns | 909 ns | 2.56 µs | 4.29 µs | 6.52 µs |
| D1232 | 194 ns | 1.35 µs | 5.08 µs | 7.02 µs | 13.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,190.6 124.4,177.4 160.5,174.3 196.7,165.8 232.9,160.9 269.1,156.6 305.3,140.6 341.5,129.7 377.6,131.6 413.8,123.4 450.0,118.5 450.0,45.3 413.8,57.4 377.6,65.1 341.5,74.5 305.3,87.0 269.1,92.9 232.9,106.1 196.7,115.6 160.5,133.1 124.4,138.5 88.2,154.1 52.0,186.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,190.6 124.4,177.4 160.5,174.3 196.7,165.8 232.9,160.9 269.1,156.6 305.3,140.6 341.5,129.7 377.6,131.6 413.8,123.4 450.0,118.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.9 88.2,163.7 124.4,156.8 160.5,146.8 196.7,141.9 232.9,144.8 269.1,127.4 305.3,123.7 341.5,107.5 377.6,101.7 413.8,91.7 450.0,84.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,152.2 124.4,148.5 160.5,144.9 196.7,132.5 232.9,134.0 269.1,107.4 305.3,111.2 341.5,85.5 377.6,80.0 413.8,73.6 450.0,61.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.3 88.2,153.3 124.4,134.7 160.5,134.0 196.7,118.2 232.9,116.6 269.1,108.3 305.3,88.8 341.5,83.2 377.6,71.5 413.8,64.7 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.5 88.2,154.1 124.4,138.5 160.5,133.1 196.7,115.6 232.9,106.1 269.1,92.9 305.3,87.0 341.5,74.5 377.6,65.1 413.8,57.4 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.622 ns | 0.703 ns | 0.622 ns | 0.545 ns |
| D38 | 1.12 ns | 1.33 ns | 1.45 ns | 1.32 ns | 1.15 ns |
| D57 | 1.68 ns | 1.68 ns | 1.74 ns | 1.74 ns | 1.47 ns |
| D76 | 2.1 ns | 2.16 ns | 2.09 ns | 2.1 ns | 1.99 ns |
| D115 | 2.63 ns | 2.83 ns | 2.83 ns | 3.28 ns | 3.29 ns |
| D153 | 4.22 ns | 3.27 ns | 2.81 ns | 3.48 ns | 4.6 ns |
| D230 | 5.16 ns | 5.99 ns | 7.24 ns | 4.32 ns | 5.62 ns |
| D307 | 12.3 ns | 9.7 ns | 5.64 ns | 12.5 ns | 7.96 ns |
| D462 | 15 ns | 14.1 ns | 15 ns | 13.9 ns | 15 ns |
| D616 | 18.8 ns | 19.3 ns | 20.1 ns | 21.8 ns | 21.9 ns |
| D924 | 60.9 ns | 65 ns | 67.8 ns | 74.2 ns | 69.1 ns |
| D1232 | 47.1 ns | 36.5 ns | 69.8 ns | 54.6 ns | 61.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,139.9 124.4,128.3 160.5,121.9 196.7,115.3 232.9,101.6 269.1,95.8 305.3,70.8 341.5,64.8 377.6,58.4 413.8,24.4 450.0,31.8 450.0,24.0 413.8,20.7 377.6,54.0 341.5,65.0 305.3,83.3 269.1,93.4 232.9,99.1 196.7,108.9 160.5,123.5 124.4,132.3 88.2,139.2 52.0,160.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,139.9 124.4,128.3 160.5,121.9 196.7,115.3 232.9,101.6 269.1,95.8 305.3,70.8 341.5,64.8 377.6,58.4 413.8,24.4 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,121.0 196.7,113.3 232.9,109.0 269.1,91.5 305.3,77.6 341.5,66.7 377.6,57.7 413.8,22.5 450.0,39.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,127.3 160.5,121.9 196.7,113.2 232.9,113.4 269.1,86.0 305.3,93.3 341.5,65.0 377.6,56.4 413.8,21.3 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,121.9 196.7,108.9 232.9,107.2 269.1,101.0 305.3,70.2 341.5,67.1 377.6,54.1 413.8,18.6 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,139.2 124.4,132.3 160.5,123.5 196.7,108.9 232.9,99.1 269.1,93.4 305.3,83.3 341.5,65.0 377.6,54.0 413.8,20.7 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.87 ns | 2.11 ns | 2.18 ns | 1.91 ns |
| D38 | 2.74 ns | 3.31 ns | 3.51 ns | 3.2 ns | 2.81 ns |
| D57 | 7.16 ns | 7.17 ns | 8.1 ns | 8.09 ns | 4.42 ns |
| D76 | 8.48 ns | 9.51 ns | 8.71 ns | 8.72 ns | 5.72 ns |
| D115 | 9.95 ns | 12.8 ns | 12.8 ns | 12.8 ns | 12.8 ns |
| D153 | 20.1 ns | 15.5 ns | 10.9 ns | 12 ns | 20 ns |
| D230 | 28 ns | 32.1 ns | 36 ns | 17 ns | 28 ns |
| D307 | 48.1 ns | 37.3 ns | 25 ns | 48 ns | 32.9 ns |
| D462 | 75.2 ns | 69.8 ns | 75.5 ns | 66.3 ns | 72.9 ns |
| D616 | 85.7 ns | 66.7 ns | 81.2 ns | 97.1 ns | 93.8 ns |
| D924 | 107 ns | 69.3 ns | 66.2 ns | 99.2 ns | 90.7 ns |
| D1232 | 132 ns | 71.7 ns | 131 ns | 102 ns | 111 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,180.9 124.4,153.0 160.5,148.1 196.7,143.5 232.9,123.2 269.1,113.5 305.3,97.9 341.5,84.9 377.6,81.1 413.8,74.7 450.0,68.6 450.0,73.6 413.8,79.5 377.6,78.5 341.5,85.8 305.3,108.8 269.1,113.5 232.9,123.2 196.7,136.3 160.5,159.5 124.4,167.0 88.2,180.1 52.0,191.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,180.9 124.4,153.0 160.5,148.1 196.7,143.5 232.9,123.2 269.1,113.5 305.3,97.9 341.5,84.9 377.6,81.1 413.8,74.7 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.4 124.4,153.0 160.5,144.8 196.7,136.3 232.9,130.6 269.1,109.6 305.3,105.2 341.5,87.1 377.6,88.4 413.8,87.3 450.0,86.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,173.6 124.4,149.4 160.5,147.3 196.7,136.3 232.9,140.8 269.1,106.2 305.3,116.8 341.5,84.8 377.6,82.7 413.8,88.6 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.3 124.4,149.5 160.5,147.3 196.7,136.3 232.9,138.1 269.1,128.0 305.3,97.9 341.5,88.6 377.6,77.5 413.8,76.9 450.0,76.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.3 88.2,180.1 124.4,167.0 160.5,159.5 196.7,136.3 232.9,123.2 269.1,113.5 305.3,108.8 341.5,85.8 377.6,78.5 413.8,79.5 450.0,73.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.94 ns | 1.05 ns | 0.937 ns | 0.819 ns |
| D38 | 1.41 ns | 1.61 ns | 1.81 ns | 1.61 ns | 1.44 ns |
| D57 | 2.25 ns | 2.25 ns | 2.5 ns | 2.5 ns | 1.92 ns |
| D76 | 3.08 ns | 3.46 ns | 3.09 ns | 3.09 ns | 2.47 ns |
| D115 | 4.48 ns | 4.89 ns | 4.87 ns | 4.9 ns | 4.88 ns |
| D153 | 8.46 ns | 6.53 ns | 5.22 ns | 6.37 ns | 8.45 ns |
| D230 | 13.7 ns | 16.1 ns | 17.7 ns | 11.3 ns | 13.7 ns |
| D307 | 25.2 ns | 19.5 ns | 14.1 ns | 25.1 ns | 19 ns |
| D462 | 38.4 ns | 34.3 ns | 38.4 ns | 33.6 ns | 38.4 ns |
| D616 | 45.6 ns | 45.9 ns | 50.2 ns | 59.3 ns | 57.1 ns |
| D924 | 83.9 ns | 67.6 ns | 68.7 ns | 80.1 ns | 76.3 ns |
| D1232 | 95.2 ns | 73.7 ns | 106 ns | 93.8 ns | 95.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,152.5 124.4,142.4 160.5,135.5 196.7,127.4 232.9,113.6 269.1,103.2 305.3,89.9 341.5,80.8 377.6,77.0 413.8,63.8 450.0,61.1 450.0,61.1 413.8,65.9 377.6,72.2 341.5,80.8 305.3,96.1 269.1,103.2 232.9,113.6 196.7,125.6 160.5,140.4 124.4,145.8 88.2,152.1 52.0,164.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,152.5 124.4,142.4 160.5,135.5 196.7,127.4 232.9,113.6 269.1,103.2 305.3,89.9 341.5,80.8 377.6,77.0 413.8,63.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.3 88.2,149.6 124.4,142.4 160.5,133.1 196.7,125.5 232.9,119.3 269.1,99.7 305.3,95.5 341.5,83.2 377.6,76.9 413.8,68.5 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,140.1 160.5,135.5 196.7,125.6 232.9,124.1 269.1,97.6 305.3,102.5 341.5,80.8 377.6,75.0 413.8,68.1 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,140.1 160.5,135.5 196.7,125.5 232.9,119.8 269.1,107.4 305.3,90.0 341.5,83.7 377.6,71.3 413.8,64.8 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,152.1 124.4,145.8 160.5,140.4 196.7,125.6 232.9,113.6 269.1,103.2 305.3,96.1 341.5,80.8 377.6,72.2 413.8,65.9 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
