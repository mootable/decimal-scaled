# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.06 ns | 0.935 ns | 0.938 ns | 0.939 ns |
| D38 | 1.82 ns | 1.84 ns | 1.82 ns | 1.83 ns | 1.61 ns |
| D57 | 3 ns | 2.89 ns | 2.25 ns | 2.5 ns | 2.25 ns |
| D76 | 3.49 ns | 3.5 ns | 3.49 ns | 2.32 ns | 3.5 ns |
| D115 | 4.4 ns | 4.4 ns | 2.31 ns | 5 ns | 3.87 ns |
| D153 | 6.63 ns | 6.62 ns | 6.62 ns | 6.63 ns | 5.95 ns |
| D230 | 13.8 ns | 15.4 ns | 15.3 ns | 12.1 ns | 11.9 ns |
| D307 | 18.5 ns | 18.6 ns | 19.6 ns | 15.2 ns | 19.6 ns |
| D462 | 29 ns | 29.6 ns | 27.3 ns | 29.1 ns | 22.9 ns |
| D616 | 44.7 ns | 70.6 ns | 62.6 ns | 57.1 ns | 45.4 ns |
| D924 | 84.8 ns | 84.9 ns | 74.5 ns | 84.9 ns | 84 ns |
| D1232 | 70.8 ns | 107 ns | 107 ns | 117 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,147.0 124.4,136.2 160.5,132.9 196.7,127.8 232.9,118.9 269.1,103.0 305.3,96.6 341.5,86.9 377.6,77.5 413.8,63.6 450.0,67.5 450.0,58.6 413.8,63.8 377.6,77.2 341.5,92.0 305.3,95.4 269.1,106.2 232.9,121.3 196.7,130.6 160.5,132.8 124.4,142.4 88.2,149.7 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,147.0 124.4,136.2 160.5,132.9 196.7,127.8 232.9,118.9 269.1,103.0 305.3,96.6 341.5,86.9 377.6,77.5 413.8,63.6 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,146.8 124.4,137.0 160.5,132.8 196.7,127.8 232.9,118.9 269.1,100.7 305.3,96.5 341.5,86.4 377.6,67.6 413.8,63.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,142.4 160.5,132.9 196.7,141.8 232.9,119.0 269.1,100.7 305.3,95.4 341.5,88.2 377.6,70.2 413.8,66.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,146.9 124.4,140.1 160.5,141.8 196.7,125.1 232.9,118.9 269.1,105.8 305.3,100.9 341.5,86.8 377.6,72.2 413.8,63.6 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,142.4 160.5,132.8 196.7,130.6 232.9,121.3 269.1,106.2 305.3,95.4 341.5,92.0 377.6,77.2 413.8,63.8 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.5 ns | 6.31 ns | 5.7 ns | 8.53 ns | 8.63 ns |
| D38 | 11.2 ns | 12.4 ns | 14.1 ns | 810 ns | 946 ns |
| D57 | 31 ns | 42.5 ns | 68.8 ns | 109 ns | 103 ns |
| D76 | 41 ns | 66.1 ns | 85.4 ns | 92.1 ns | 132 ns |
| D115 | 54.8 ns | 82.3 ns | 60.4 ns | 186 ns | 178 ns |
| D153 | 67.9 ns | 114 ns | 153 ns | 239 ns | 299 ns |
| D230 | 95.7 ns | 168 ns | 250 ns | 371 ns | 462 ns |
| D307 | 130 ns | 220 ns | 399 ns | 477 ns | 943 ns |
| D462 | 211 ns | 422 ns | 564 ns | 1.03 µs | 1.41 µs |
| D616 | 265 ns | 646 ns | 1.01 µs | 1.92 µs | 2.19 µs |
| D924 | 379 ns | 1.19 µs | 2.02 µs | 2.78 µs | 4.48 µs |
| D1232 | 402 ns | 1.9 µs | 3.75 µs | 4.6 µs | 7.84 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,157.6 124.4,135.4 160.5,129.4 196.7,123.0 232.9,118.4 269.1,111.0 305.3,104.2 341.5,93.8 377.6,88.9 413.8,81.0 450.0,79.8 450.0,15.3 413.8,27.4 377.6,42.9 341.5,52.5 305.3,61.3 269.1,76.7 232.9,86.2 196.7,97.4 160.5,103.9 124.4,109.5 88.2,61.2 52.0,163.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,157.6 124.4,135.4 160.5,129.4 196.7,123.0 232.9,118.4 269.1,111.0 305.3,104.2 341.5,93.8 377.6,88.9 413.8,81.0 450.0,79.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,155.4 124.4,128.6 160.5,119.0 196.7,114.2 232.9,107.2 269.1,98.8 305.3,92.9 341.5,78.7 377.6,69.5 413.8,56.3 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,152.6 124.4,118.1 160.5,113.4 196.7,120.9 232.9,100.7 269.1,90.1 305.3,80.0 341.5,72.4 377.6,59.8 413.8,44.7 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.4 88.2,64.6 124.4,108.2 160.5,111.8 196.7,96.5 232.9,91.1 269.1,81.5 305.3,76.1 341.5,59.3 377.6,45.9 413.8,37.8 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.2 88.2,61.2 124.4,109.5 160.5,103.9 196.7,97.4 232.9,86.2 269.1,76.7 305.3,61.3 341.5,52.5 377.6,42.9 413.8,27.4 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3.07 ns | 3.1 ns | 4.92 ns | 5.09 ns |
| D38 | 3.94 ns | 13.7 ns | 30.5 ns | 33.9 ns | 28.5 ns |
| D57 | 4.25 ns | 21.1 ns | 33 ns | 78.4 ns | 71.7 ns |
| D76 | 5.64 ns | 37.7 ns | 43.6 ns | 74.8 ns | 107 ns |
| D115 | 13.2 ns | 47.6 ns | 45.3 ns | 214 ns | 195 ns |
| D153 | 16.9 ns | 57.9 ns | 121 ns | 258 ns | 353 ns |
| D230 | 27.2 ns | 125 ns | 371 ns | 488 ns | 825 ns |
| D307 | 44.4 ns | 169 ns | 514 ns | 875 ns | 1.46 µs |
| D462 | 77 ns | 436 ns | 1.06 µs | 1.75 µs | 2.35 µs |
| D616 | 91 ns | 737 ns | 1.75 µs | 2.91 µs | 3.87 µs |
| D924 | 163 ns | 1.59 µs | 2.95 µs | 5.4 µs | 7.61 µs |
| D1232 | 150 ns | 2.42 µs | 5.04 µs | 8.92 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.2 124.4,184.9 160.5,179.9 196.7,165.2 232.9,160.9 269.1,152.6 305.3,144.1 341.5,134.5 377.6,131.6 413.8,121.5 450.0,123.0 450.0,44.0 413.8,54.7 377.6,66.5 341.5,75.1 305.3,83.5 269.1,93.3 232.9,108.1 196.7,118.4 160.5,128.8 124.4,135.8 88.2,151.8 52.0,181.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.2 124.4,184.9 160.5,179.9 196.7,165.2 232.9,160.9 269.1,152.6 305.3,144.1 341.5,134.5 377.6,131.6 413.8,121.5 450.0,123.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.5 88.2,164.5 124.4,157.0 160.5,146.9 196.7,142.9 232.9,139.5 269.1,126.1 305.3,120.9 341.5,104.4 377.6,95.3 413.8,81.9 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.3 88.2,150.6 124.4,149.3 160.5,144.4 196.7,143.8 232.9,126.7 269.1,107.2 305.3,101.6 341.5,89.0 377.6,80.3 413.8,71.2 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.3 88.2,148.8 124.4,134.2 160.5,135.0 196.7,116.8 232.9,113.5 269.1,102.5 305.3,92.3 341.5,80.3 377.6,71.5 413.8,60.7 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.7 88.2,151.8 124.4,135.8 160.5,128.8 196.7,118.4 232.9,108.1 269.1,93.3 305.3,83.5 341.5,75.1 377.6,66.5 413.8,54.7 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.703 ns | 0.622 ns | 0.622 ns | 0.622 ns |
| D38 | 1.45 ns | 1.45 ns | 1.45 ns | 1.45 ns | 1.33 ns |
| D57 | 1.57 ns | 1.57 ns | 1.87 ns | 1.74 ns | 1.87 ns |
| D76 | 2.16 ns | 2.16 ns | 2.17 ns | 1.8 ns | 2.63 ns |
| D115 | 2.86 ns | 2.86 ns | 1.46 ns | 3.55 ns | 2.75 ns |
| D153 | 4.22 ns | 4.22 ns | 4.6 ns | 4.6 ns | 4.29 ns |
| D230 | 5.86 ns | 6.65 ns | 7.24 ns | 5.23 ns | 5.62 ns |
| D307 | 10.9 ns | 11.1 ns | 12.5 ns | 9.69 ns | 12.5 ns |
| D462 | 15 ns | 15.3 ns | 14 ns | 14.9 ns | 10.9 ns |
| D616 | 19 ns | 21.9 ns | 20.1 ns | 21.9 ns | 20.2 ns |
| D924 | 63.5 ns | 84.8 ns | 75.8 ns | 85 ns | 83.5 ns |
| D1232 | 33.7 ns | 69.8 ns | 69.7 ns | 79.2 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.6 124.4,130.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,92.1 305.3,74.2 341.5,64.9 377.6,58.1 413.8,23.2 450.0,41.5 450.0,20.4 413.8,15.2 377.6,56.4 341.5,74.1 305.3,70.2 269.1,93.3 232.9,101.1 196.7,114.0 160.5,115.4 124.4,125.2 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.6 124.4,130.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,92.1 305.3,74.2 341.5,64.9 377.6,58.1 413.8,23.2 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,132.6 124.4,130.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,88.5 305.3,73.7 341.5,64.4 377.6,53.9 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,120.9 196.7,132.3 232.9,99.1 269.1,86.0 305.3,70.1 341.5,67.0 377.6,56.4 413.8,18.0 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,127.3 160.5,126.3 196.7,106.7 232.9,99.1 269.1,95.4 305.3,77.6 341.5,65.1 377.6,53.9 413.8,14.7 450.0,16.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,115.4 196.7,114.0 232.9,101.1 269.1,93.3 305.3,70.2 341.5,74.1 377.6,56.4 413.8,15.2 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.11 ns | 1.87 ns | 2.18 ns | 2.18 ns |
| D38 | 7.96 ns | 8.18 ns | 14.8 ns | 19.3 ns | 13.1 ns |
| D57 | 6.2 ns | 6.18 ns | 7.16 ns | 8.09 ns | 7.16 ns |
| D76 | 9.83 ns | 9.5 ns | 9.51 ns | 6.75 ns | 9.72 ns |
| D115 | 12.8 ns | 12.4 ns | 6.15 ns | 14.1 ns | 10.9 ns |
| D153 | 20.7 ns | 20 ns | 20.1 ns | 20.1 ns | 16.4 ns |
| D230 | 32.2 ns | 36.3 ns | 36.2 ns | 22.9 ns | 28.2 ns |
| D307 | 41.3 ns | 40.1 ns | 63.3 ns | 37.2 ns | 47.8 ns |
| D462 | 75.6 ns | 85 ns | 67.6 ns | 79.8 ns | 53.5 ns |
| D616 | 83.1 ns | 101 ns | 83.9 ns | 97.7 ns | 77.5 ns |
| D924 | 121 ns | 114 ns | 99.8 ns | 102 ns | 104 ns |
| D1232 | 103 ns | 137 ns | 129 ns | 145 ns | 126 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,149.9 124.4,157.2 160.5,143.8 196.7,136.3 232.9,122.2 269.1,109.5 305.3,102.3 341.5,84.8 377.6,82.0 413.8,71.2 450.0,75.7 450.0,70.1 413.8,75.5 377.6,84.0 341.5,94.8 305.3,98.0 269.1,113.3 232.9,129.0 196.7,140.8 160.5,144.2 124.4,153.0 88.2,135.5 52.0,187.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,149.9 124.4,157.2 160.5,143.8 196.7,136.3 232.9,122.2 269.1,109.5 305.3,102.3 341.5,84.8 377.6,82.0 413.8,71.2 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,149.1 124.4,157.2 160.5,144.8 196.7,137.0 232.9,123.2 269.1,106.0 305.3,103.1 341.5,81.4 377.6,76.3 413.8,72.8 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,132.0 124.4,153.0 160.5,144.8 196.7,157.4 232.9,123.2 269.1,106.1 305.3,89.9 341.5,88.0 377.6,81.8 413.8,76.7 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,124.3 124.4,149.5 160.5,154.7 196.7,133.5 232.9,123.2 269.1,119.3 305.3,105.3 341.5,83.2 377.6,77.3 413.8,76.2 450.0,65.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,135.5 124.4,153.0 160.5,144.2 196.7,140.8 232.9,129.0 269.1,113.3 305.3,98.0 341.5,94.8 377.6,84.0 413.8,75.5 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.06 ns | 0.934 ns | 0.938 ns | 0.936 ns |
| D38 | 1.82 ns | 1.82 ns | 1.82 ns | 1.82 ns | 1.6 ns |
| D57 | 2.93 ns | 2.93 ns | 2.25 ns | 2.5 ns | 2.25 ns |
| D76 | 3.44 ns | 3.46 ns | 3.46 ns | 2.46 ns | 3.45 ns |
| D115 | 4.84 ns | 4.85 ns | 2.9 ns | 5.55 ns | 4.31 ns |
| D153 | 8.47 ns | 8.46 ns | 8.47 ns | 8.47 ns | 7.65 ns |
| D230 | 16.1 ns | 17.7 ns | 17.7 ns | 13.7 ns | 13.7 ns |
| D307 | 23.5 ns | 23.5 ns | 25.2 ns | 19.5 ns | 25.1 ns |
| D462 | 37 ns | 37.4 ns | 34 ns | 37.1 ns | 29.7 ns |
| D616 | 45.9 ns | 70.9 ns | 63 ns | 50.2 ns | 46.1 ns |
| D924 | 84.9 ns | 84.7 ns | 74.8 ns | 84.9 ns | 83.9 ns |
| D1232 | 77.7 ns | 106 ns | 106 ns | 117 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,146.9 124.4,136.7 160.5,133.2 196.7,125.8 232.9,113.6 269.1,99.6 305.3,91.5 341.5,81.6 377.6,76.9 413.8,63.6 450.0,65.5 450.0,58.7 413.8,63.8 377.6,76.8 341.5,86.4 305.3,90.0 269.1,103.1 232.9,115.8 196.7,128.3 160.5,133.1 124.4,142.4 88.2,149.8 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,146.9 124.4,136.7 160.5,133.2 196.7,125.8 232.9,113.6 269.1,99.6 305.3,91.5 341.5,81.6 377.6,76.9 413.8,63.6 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,147.0 124.4,136.7 160.5,133.1 196.7,125.7 232.9,113.6 269.1,97.7 305.3,91.4 341.5,81.4 377.6,67.5 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,133.1 196.7,136.9 232.9,113.6 269.1,97.7 305.3,90.0 341.5,83.4 377.6,70.0 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,140.1 160.5,140.4 196.7,122.8 232.9,113.6 269.1,103.1 305.3,95.5 341.5,81.5 377.6,75.0 413.8,63.6 450.0,56.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.8 124.4,142.4 160.5,133.1 196.7,128.3 232.9,115.8 269.1,103.1 305.3,90.0 341.5,86.4 377.6,76.8 413.8,63.8 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
