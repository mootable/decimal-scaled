# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.06 ns | 0.821 ns | 0.936 ns | 0.935 ns |
| D38 | 1.83 ns | 1.63 ns | 1.83 ns | 1.82 ns | 1.83 ns |
| D57 | 2.5 ns | 2.5 ns | 2.25 ns | 2.25 ns | 2.49 ns |
| D76 | 3.48 ns | 3.5 ns | 3.09 ns | 2.71 ns | 3.08 ns |
| D115 | 4.41 ns | 4.99 ns | 5.01 ns | 3.88 ns | 2.44 ns |
| D153 | 6.62 ns | 6.62 ns | 6.64 ns | 6.62 ns | 5.96 ns |
| D230 | 15.4 ns | 13.9 ns | 15.4 ns | 12.1 ns | 13.9 ns |
| D307 | 18.5 ns | 18.6 ns | 18.5 ns | 18.6 ns | 19.6 ns |
| D462 | 28.9 ns | 22.9 ns | 29.1 ns | 32.5 ns | 30 ns |
| D616 | 61.3 ns | 62.5 ns | 56 ns | 45.2 ns | 45.3 ns |
| D924 | 84.7 ns | 84.7 ns | 75.7 ns | 85 ns | 75 ns |
| D1232 | 107 ns | 107 ns | 107 ns | 70.7 ns | 108 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,146.9 124.4,140.1 160.5,132.9 196.7,127.8 232.9,118.9 269.1,100.6 305.3,96.6 341.5,86.9 377.6,70.6 413.8,63.6 450.0,58.6 450.0,58.4 413.8,66.3 377.6,77.2 341.5,86.1 305.3,95.4 269.1,102.9 232.9,121.2 196.7,140.7 160.5,135.6 124.4,140.2 88.2,146.9 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,146.9 124.4,140.1 160.5,132.9 196.7,127.8 232.9,118.9 269.1,100.6 305.3,96.6 341.5,86.9 377.6,70.6 413.8,63.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.4 124.4,140.1 160.5,132.8 196.7,125.1 232.9,118.9 269.1,102.9 305.3,96.6 341.5,92.0 377.6,70.2 413.8,63.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,146.9 124.4,142.4 160.5,135.5 196.7,125.0 232.9,118.9 269.1,100.6 305.3,96.6 341.5,86.8 377.6,72.6 413.8,66.1 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,142.4 160.5,138.4 196.7,130.6 232.9,118.9 269.1,105.8 305.3,96.6 341.5,84.4 377.6,77.2 413.8,63.5 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.9 124.4,140.2 160.5,135.6 196.7,140.7 232.9,121.2 269.1,102.9 305.3,95.4 341.5,86.1 377.6,77.2 413.8,66.3 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.52 ns | 6.32 ns | 4.92 ns | 8.53 ns | 8.6 ns |
| D38 | 11.2 ns | 11 ns | 14.2 ns | 809 ns | 1.07 µs |
| D57 | 35.3 ns | 52 ns | 65.2 ns | 101 ns | 108 ns |
| D76 | 40.9 ns | 65.9 ns | 76.6 ns | 93.6 ns | 125 ns |
| D115 | 54.9 ns | 84.3 ns | 116 ns | 144 ns | 146 ns |
| D153 | 68 ns | 114 ns | 152 ns | 239 ns | 299 ns |
| D230 | 109 ns | 153 ns | 248 ns | 382 ns | 535 ns |
| D307 | 130 ns | 219 ns | 358 ns | 559 ns | 946 ns |
| D462 | 228 ns | 339 ns | 655 ns | 1.1 µs | 1.35 µs |
| D616 | 258 ns | 640 ns | 968 ns | 1.76 µs | 2.19 µs |
| D924 | 383 ns | 1.18 µs | 2.02 µs | 2.83 µs | 4.46 µs |
| D1232 | 527 ns | 1.91 µs | 3.77 µs | 4.42 µs | 7.86 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.3 88.2,157.6 124.4,132.6 160.5,129.4 196.7,123.0 232.9,118.4 269.1,108.1 305.3,104.2 341.5,92.1 377.6,89.4 413.8,80.8 450.0,73.9 450.0,15.2 413.8,27.5 377.6,43.0 341.5,53.5 305.3,61.2 269.1,73.6 232.9,86.2 196.7,101.8 160.5,105.1 124.4,108.4 88.2,58.6 52.0,163.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.3 88.2,157.6 124.4,132.6 160.5,129.4 196.7,123.0 232.9,118.4 269.1,108.1 305.3,104.2 341.5,92.1 377.6,89.4 413.8,80.8 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,157.9 124.4,124.2 160.5,119.1 196.7,113.7 232.9,107.1 269.1,100.7 305.3,93.0 341.5,83.5 377.6,69.7 413.8,56.5 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.4 88.2,152.4 124.4,119.3 160.5,115.8 196.7,106.8 232.9,100.8 269.1,90.3 305.3,82.3 341.5,69.2 377.6,60.7 413.8,44.7 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.4 88.2,64.6 124.4,109.9 160.5,111.4 196.7,102.0 232.9,91.1 269.1,80.9 305.3,72.6 341.5,57.8 377.6,47.7 413.8,37.4 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.3 88.2,58.6 124.4,108.4 160.5,105.1 196.7,101.8 232.9,86.2 269.1,73.6 305.3,61.2 341.5,53.5 377.6,43.0 413.8,27.5 450.0,15.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3.06 ns | 2.49 ns | 4.9 ns | 5.09 ns |
| D38 | 3.94 ns | 13.7 ns | 30.1 ns | 33.7 ns | 29.5 ns |
| D57 | 4.22 ns | 24.3 ns | 32.9 ns | 71.5 ns | 77.3 ns |
| D76 | 5.64 ns | 38.3 ns | 42.2 ns | 66.4 ns | 103 ns |
| D115 | 13.2 ns | 57.5 ns | 89.6 ns | 166 ns | 153 ns |
| D153 | 16.8 ns | 57.4 ns | 121 ns | 258 ns | 353 ns |
| D230 | 27.9 ns | 122 ns | 371 ns | 488 ns | 979 ns |
| D307 | 44.4 ns | 169 ns | 459 ns | 1.05 µs | 1.46 µs |
| D462 | 76.8 ns | 366 ns | 1.24 µs | 1.85 µs | 2.43 µs |
| D616 | 104 ns | 726 ns | 1.66 µs | 2.7 µs | 3.88 µs |
| D924 | 159 ns | 1.58 µs | 2.95 µs | 5.4 µs | 7.62 µs |
| D1232 | 198 ns | 2.39 µs | 5.03 µs | 8.14 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.2 124.4,185.0 160.5,179.9 196.7,165.2 232.9,160.9 269.1,152.2 305.3,144.1 341.5,134.6 377.6,129.4 413.8,122.0 450.0,118.1 450.0,44.0 413.8,54.7 377.6,66.5 341.5,74.6 305.3,83.4 269.1,90.4 232.9,108.1 196.7,122.6 160.5,129.5 124.4,134.5 88.2,151.2 52.0,181.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.2 124.4,185.0 160.5,179.9 196.7,165.2 232.9,160.9 269.1,152.2 305.3,144.1 341.5,134.6 377.6,129.4 413.8,122.0 450.0,118.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.5 88.2,164.5 124.4,154.5 160.5,146.7 196.7,139.6 232.9,139.6 269.1,126.6 305.3,120.9 341.5,107.5 377.6,95.6 413.8,82.0 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.2 88.2,150.8 124.4,149.3 160.5,145.0 196.7,131.9 232.9,126.7 269.1,107.2 305.3,103.5 341.5,86.2 377.6,81.2 413.8,71.2 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.4 88.2,148.9 124.4,135.8 160.5,137.1 196.7,121.2 232.9,113.5 269.1,102.4 305.3,89.2 341.5,79.3 377.6,72.7 413.8,60.7 450.0,53.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.7 88.2,151.2 124.4,134.5 160.5,129.5 196.7,122.6 232.9,108.1 269.1,90.4 305.3,83.4 341.5,74.6 377.6,66.5 413.8,54.7 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.703 ns | 0.545 ns | 0.622 ns | 0.622 ns |
| D38 | 1.45 ns | 1.33 ns | 1.45 ns | 1.45 ns | 1.45 ns |
| D57 | 1.74 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.74 ns |
| D76 | 2.17 ns | 2.17 ns | 2.09 ns | 1.67 ns | 2.49 ns |
| D115 | 2.86 ns | 3.17 ns | 3.17 ns | 2.75 ns | 2.06 ns |
| D153 | 4.22 ns | 4.22 ns | 4.6 ns | 4.6 ns | 4.3 ns |
| D230 | 6.66 ns | 5.86 ns | 7.23 ns | 5.23 ns | 7.16 ns |
| D307 | 10.9 ns | 11.1 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 15 ns | 10.8 ns | 14.9 ns | 16.7 ns | 15.3 ns |
| D616 | 23.6 ns | 21.8 ns | 15 ns | 20.2 ns | 19.8 ns |
| D924 | 63.3 ns | 84.7 ns | 75.4 ns | 84.7 ns | 75.6 ns |
| D1232 | 54.3 ns | 69.9 ns | 69.9 ns | 43.9 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.6 124.4,127.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,88.4 305.3,74.2 341.5,64.9 377.6,51.8 413.8,23.2 450.0,27.7 450.0,20.4 413.8,18.1 377.6,56.9 341.5,64.4 305.3,70.2 269.1,86.4 232.9,101.1 196.7,122.4 160.5,116.9 124.4,127.3 88.2,132.6 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.6 124.4,127.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,88.4 305.3,74.2 341.5,64.9 377.6,51.8 413.8,23.2 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,120.9 196.7,110.0 232.9,101.7 269.1,92.1 305.3,73.7 341.5,74.3 377.6,54.2 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,132.6 124.4,125.2 160.5,121.9 196.7,110.0 232.9,99.1 269.1,86.0 305.3,73.7 341.5,65.1 377.6,64.9 413.8,18.2 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,128.4 196.7,114.0 232.9,99.2 269.1,95.5 305.3,73.7 341.5,61.9 377.6,56.3 413.8,14.8 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,127.3 160.5,116.9 196.7,122.4 232.9,101.1 269.1,86.4 305.3,70.2 341.5,64.4 377.6,56.9 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.11 ns | 1.64 ns | 2.18 ns | 2.18 ns |
| D38 | 8.1 ns | 7.43 ns | 14.7 ns | 19.4 ns | 16.5 ns |
| D57 | 8.09 ns | 8.09 ns | 7.16 ns | 7.16 ns | 8.08 ns |
| D76 | 9.83 ns | 9.84 ns | 8.61 ns | 7.43 ns | 8.49 ns |
| D115 | 12.8 ns | 14.1 ns | 14.2 ns | 10.9 ns | 6.94 ns |
| D153 | 20.7 ns | 20 ns | 20 ns | 20.1 ns | 16.4 ns |
| D230 | 39.4 ns | 32.3 ns | 36.3 ns | 22.9 ns | 32.1 ns |
| D307 | 41.3 ns | 40.2 ns | 44.2 ns | 43.5 ns | 48.5 ns |
| D462 | 78.8 ns | 54.9 ns | 73.6 ns | 84.7 ns | 79.7 ns |
| D616 | 102 ns | 99.2 ns | 70.5 ns | 77.7 ns | 77 ns |
| D924 | 119 ns | 115 ns | 101 ns | 101 ns | 94.4 ns |
| D1232 | 144 ns | 137 ns | 131 ns | 84.3 ns | 123 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,149.4 124.4,149.5 160.5,143.8 196.7,136.3 232.9,122.2 269.1,103.6 305.3,102.3 341.5,83.6 377.6,76.0 413.8,71.6 450.0,66.2 450.0,70.7 413.8,78.3 377.6,84.2 341.5,83.2 305.3,97.6 269.1,109.6 232.9,129.0 196.7,153.9 160.5,148.1 124.4,149.5 88.2,128.8 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,149.4 124.4,149.5 160.5,143.8 196.7,136.3 232.9,122.2 269.1,103.6 305.3,102.3 341.5,83.6 377.6,76.0 413.8,71.6 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,151.9 124.4,149.5 160.5,143.8 196.7,133.5 232.9,123.2 269.1,109.4 305.3,103.1 341.5,94.1 377.6,76.9 413.8,72.7 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.8 88.2,132.2 124.4,153.0 160.5,147.7 196.7,133.1 232.9,123.2 269.1,106.0 305.3,100.3 341.5,85.5 377.6,86.8 413.8,76.3 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,124.2 124.4,153.0 160.5,151.9 196.7,140.8 232.9,123.2 269.1,119.3 305.3,100.8 341.5,81.5 377.6,84.0 413.8,76.3 450.0,81.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,128.8 124.4,149.5 160.5,148.1 196.7,153.9 232.9,129.0 269.1,109.6 305.3,97.6 341.5,83.2 377.6,84.2 413.8,78.3 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.06 ns | 0.818 ns | 0.935 ns | 0.935 ns |
| D38 | 1.82 ns | 1.61 ns | 1.82 ns | 1.81 ns | 1.82 ns |
| D57 | 2.5 ns | 2.5 ns | 2.25 ns | 2.25 ns | 2.5 ns |
| D76 | 3.46 ns | 3.46 ns | 3.09 ns | 2.68 ns | 3.09 ns |
| D115 | 4.86 ns | 5.55 ns | 5.56 ns | 4.3 ns | 3.15 ns |
| D153 | 8.46 ns | 8.46 ns | 8.45 ns | 8.48 ns | 7.64 ns |
| D230 | 17.6 ns | 16.2 ns | 17.6 ns | 13.7 ns | 16.1 ns |
| D307 | 23.5 ns | 23.5 ns | 23.5 ns | 23.5 ns | 25.2 ns |
| D462 | 38.9 ns | 29.5 ns | 38.9 ns | 40.5 ns | 37.6 ns |
| D616 | 62.6 ns | 63.5 ns | 56.8 ns | 45.9 ns | 46.1 ns |
| D924 | 84.9 ns | 84.7 ns | 79.2 ns | 84.8 ns | 76.1 ns |
| D1232 | 106 ns | 106 ns | 107 ns | 77.7 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,133.0 196.7,125.7 232.9,113.6 269.1,97.7 305.3,91.5 341.5,80.5 377.6,70.2 413.8,63.6 450.0,58.7 450.0,58.6 413.8,65.9 377.6,76.8 341.5,81.3 305.3,90.0 269.1,99.6 232.9,115.8 196.7,135.1 160.5,135.5 124.4,140.1 88.2,147.0 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,133.0 196.7,125.7 232.9,113.6 269.1,97.7 305.3,91.5 341.5,80.5 377.6,70.2 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.7 124.4,140.1 160.5,133.1 196.7,122.8 232.9,113.6 269.1,99.5 305.3,91.4 341.5,86.5 377.6,69.9 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.4 88.2,147.0 124.4,142.4 160.5,135.5 196.7,122.7 232.9,113.7 269.1,97.7 305.3,91.5 341.5,80.5 377.6,72.3 413.8,65.1 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.1 124.4,142.4 160.5,138.6 196.7,128.3 232.9,113.6 269.1,103.2 305.3,91.5 341.5,79.6 377.6,76.9 413.8,63.6 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.0 124.4,140.1 160.5,135.5 196.7,135.1 232.9,115.8 269.1,99.6 305.3,90.0 341.5,81.3 377.6,76.8 413.8,65.9 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
