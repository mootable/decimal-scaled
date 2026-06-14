# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 1.06 ns | 0.941 ns | 0.936 ns | 1.05 ns |
| D38 | 1.82 ns | 1.82 ns | 1.82 ns | 1.63 ns | 1.62 ns |
| D57 | 1.94 ns | 2.26 ns | 2.25 ns | 2.5 ns | 2.5 ns |
| D76 | 3.08 ns | 3.1 ns | 3.08 ns | 3.5 ns | 3.48 ns |
| D115 | 4.99 ns | 5.01 ns | 4.41 ns | 4.38 ns | 5 ns |
| D153 | 4.47 ns | 5.94 ns | 6.65 ns | 3.15 ns | 6.63 ns |
| D230 | 13.8 ns | 15.4 ns | 15.4 ns | 13.8 ns | 13.8 ns |
| D307 | 18.5 ns | 19.6 ns | 14.6 ns | 18.5 ns | 18.5 ns |
| D462 | 33.3 ns | 32.6 ns | 28.9 ns | 26 ns | 28.9 ns |
| D616 | 51.9 ns | 59.5 ns | 60.2 ns | 45.2 ns | 54.9 ns |
| D924 | 75.3 ns | 74.9 ns | 75 ns | 97.7 ns | 74.4 ns |
| D1232 | 107 ns | 59.3 ns | 107 ns | 95 ns | 94.3 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,147.0 124.4,145.7 160.5,135.6 196.7,125.1 232.9,127.5 269.1,103.0 305.3,96.6 341.5,83.9 377.6,74.3 413.8,66.2 450.0,58.5 450.0,61.3 413.8,66.4 377.6,73.0 341.5,86.9 305.3,96.6 269.1,103.0 232.9,118.9 196.7,125.1 160.5,132.9 124.4,140.1 88.2,149.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,147.0 124.4,145.7 160.5,135.6 196.7,125.1 232.9,127.5 269.1,103.0 305.3,96.6 341.5,83.9 377.6,74.3 413.8,66.2 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,147.0 124.4,142.3 160.5,135.4 196.7,125.0 232.9,121.3 269.1,100.6 305.3,95.4 341.5,84.4 377.6,71.3 413.8,66.3 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,147.0 124.4,142.4 160.5,135.6 196.7,127.8 232.9,118.8 269.1,100.6 305.3,101.8 341.5,86.9 377.6,71.0 413.8,66.2 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.4 124.4,140.1 160.5,132.8 196.7,127.9 232.9,135.1 269.1,102.9 305.3,96.6 341.5,89.3 377.6,77.2 413.8,60.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,140.1 160.5,132.9 196.7,125.1 232.9,118.9 269.1,103.0 305.3,96.6 341.5,86.9 377.6,73.0 413.8,66.4 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.96 ns | 6.31 ns | 5.71 ns | 8.54 ns | 9.32 ns |
| D38 | 11.2 ns | 12.4 ns | 14.1 ns | 726 ns | 946 ns |
| D57 | 29 ns | 49.2 ns | 66.8 ns | 107 ns | 107 ns |
| D76 | 40.3 ns | 61.6 ns | 76.1 ns | 105 ns | 133 ns |
| D115 | 57.2 ns | 84.3 ns | 104 ns | 168 ns | 229 ns |
| D153 | 60.6 ns | 109 ns | 153 ns | 139 ns | 319 ns |
| D230 | 95.5 ns | 168 ns | 250 ns | 364 ns | 540 ns |
| D307 | 130 ns | 240 ns | 333 ns | 564 ns | 846 ns |
| D462 | 234 ns | 464 ns | 656 ns | 916 ns | 1.34 µs |
| D616 | 259 ns | 644 ns | 1.06 µs | 1.76 µs | 2.2 µs |
| D924 | 350 ns | 1.05 µs | 2.02 µs | 2.82 µs | 4.44 µs |
| D1232 | 594 ns | 1.04 µs | 3.78 µs | 4.14 µs | 6.16 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.2 88.2,157.6 124.4,136.9 160.5,129.7 196.7,122.1 232.9,120.9 269.1,111.0 305.3,104.2 341.5,91.5 377.6,89.3 413.8,82.8 450.0,71.3 450.0,20.5 413.8,27.6 377.6,42.9 341.5,53.7 305.3,63.6 269.1,73.4 232.9,84.8 196.7,92.0 160.5,103.8 124.4,108.6 88.2,61.2 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,157.6 124.4,136.9 160.5,129.7 196.7,122.1 232.9,120.9 269.1,111.0 305.3,104.2 341.5,91.5 377.6,89.3 413.8,82.8 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,155.4 124.4,125.4 160.5,120.5 196.7,113.7 232.9,108.2 269.1,98.7 305.3,91.0 341.5,76.7 377.6,69.6 413.8,59.0 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,152.6 124.4,118.8 160.5,115.9 196.7,109.1 232.9,100.8 269.1,90.1 305.3,83.9 341.5,69.2 377.6,58.7 413.8,44.7 450.0,31.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.4 88.2,66.9 124.4,108.5 160.5,108.9 196.7,98.7 232.9,102.8 269.1,81.9 305.3,72.4 341.5,61.9 377.6,47.7 413.8,37.5 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,61.2 124.4,108.6 160.5,103.8 196.7,92.0 232.9,84.8 269.1,73.4 305.3,63.6 341.5,53.7 377.6,42.9 413.8,27.6 450.0,20.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.886 ns | 3.07 ns | 3.1 ns | 4.92 ns | 4.96 ns |
| D38 | 3.94 ns | 13.7 ns | 30.3 ns | 25.1 ns | 28.5 ns |
| D57 | 3.28 ns | 20.8 ns | 32.8 ns | 78.2 ns | 77.4 ns |
| D76 | 8.1 ns | 34.2 ns | 42.1 ns | 85.5 ns | 107 ns |
| D115 | 13.6 ns | 57.1 ns | 94 ns | 194 ns | 252 ns |
| D153 | 14.8 ns | 52.6 ns | 120 ns | 136 ns | 396 ns |
| D230 | 27.2 ns | 125 ns | 371 ns | 517 ns | 978 ns |
| D307 | 44.4 ns | 188 ns | 417 ns | 1.04 µs | 1.39 µs |
| D462 | 91.7 ns | 472 ns | 1.27 µs | 1.47 µs | 2.44 µs |
| D616 | 102 ns | 736 ns | 1.84 µs | 2.7 µs | 3.87 µs |
| D924 | 146 ns | 1.48 µs | 2.97 µs | 5.43 µs | 7.62 µs |
| D1232 | 201 ns | 1.23 µs | 5.06 µs | 8.11 µs | 11 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,156.8 124.4,159.5 160.5,146.4 196.7,138.9 232.9,137.7 269.1,128.9 305.3,121.8 341.5,111.2 377.6,109.7 413.8,104.5 450.0,99.9 450.0,42.0 413.8,47.3 377.6,57.1 341.5,63.8 305.3,71.9 269.1,77.0 232.9,90.1 196.7,96.6 160.5,109.0 124.4,113.7 88.2,128.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,156.8 124.4,159.5 160.5,146.4 196.7,138.9 232.9,137.7 269.1,128.9 305.3,121.8 341.5,111.2 377.6,109.7 413.8,104.5 450.0,99.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.4 88.2,138.8 124.4,132.7 160.5,125.5 196.7,118.1 232.9,119.3 269.1,106.7 305.3,100.9 341.5,87.5 377.6,81.1 413.8,71.0 450.0,73.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.3 88.2,127.3 124.4,126.1 160.5,122.5 196.7,110.9 232.9,107.3 269.1,91.0 305.3,89.3 341.5,73.2 377.6,67.8 413.8,60.9 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,130.0 124.4,113.6 160.5,112.3 196.7,100.4 232.9,105.6 269.1,86.2 305.3,76.1 341.5,71.1 377.6,62.3 413.8,52.2 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,128.2 124.4,113.7 160.5,109.0 196.7,96.6 232.9,90.1 269.1,77.0 305.3,71.9 341.5,63.8 377.6,57.1 413.8,47.3 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.703 ns | 0.622 ns | 0.622 ns | 0.703 ns |
| D38 | 1.45 ns | 1.45 ns | 1.45 ns | 1.33 ns | 1.33 ns |
| D57 | 1.35 ns | 1.87 ns | 1.87 ns | 1.74 ns | 1.74 ns |
| D76 | 2.11 ns | 2.09 ns | 2.09 ns | 2.17 ns | 2.63 ns |
| D115 | 3.16 ns | 3.16 ns | 2.86 ns | 3.25 ns | 3.56 ns |
| D153 | 2.9 ns | 3.82 ns | 4.61 ns | 2.31 ns | 4.6 ns |
| D230 | 5.86 ns | 6.65 ns | 7.24 ns | 7.16 ns | 7.16 ns |
| D307 | 10.9 ns | 12.5 ns | 7.76 ns | 11.1 ns | 11.1 ns |
| D462 | 17.5 ns | 16.6 ns | 14.9 ns | 13.4 ns | 14.9 ns |
| D616 | 23 ns | 21.7 ns | 21.6 ns | 19.9 ns | 20.1 ns |
| D924 | 54.8 ns | 76.1 ns | 75.9 ns | 84.8 ns | 75.6 ns |
| D1232 | 54.3 ns | 35.6 ns | 69.8 ns | 61.7 ns | 57.3 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,132.7 124.4,134.7 160.5,121.8 196.7,110.0 232.9,112.6 269.1,92.1 305.3,74.2 341.5,60.4 377.6,52.5 413.8,27.4 450.0,27.7 450.0,26.1 413.8,18.1 377.6,56.4 341.5,65.1 305.3,73.7 269.1,86.3 232.9,99.2 196.7,106.6 160.5,115.4 124.4,127.3 88.2,135.1 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,132.7 124.4,134.7 160.5,121.8 196.7,110.0 232.9,112.6 269.1,92.1 305.3,74.2 341.5,60.4 377.6,52.5 413.8,27.4 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,132.6 124.4,125.2 160.5,122.0 196.7,110.0 232.9,104.5 269.1,88.5 305.3,70.2 341.5,61.9 377.6,54.2 413.8,17.9 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,122.0 196.7,112.9 232.9,99.1 269.1,86.0 305.3,84.0 341.5,65.1 377.6,54.3 413.8,18.0 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,127.3 160.5,120.9 196.7,109.2 232.9,119.0 269.1,86.3 305.3,73.7 341.5,68.3 377.6,56.8 413.8,14.8 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.1 124.4,127.3 160.5,115.4 196.7,106.6 232.9,99.2 269.1,86.3 305.3,73.7 341.5,65.1 377.6,56.4 413.8,18.1 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 2.11 ns | 1.87 ns | 2.18 ns | 2.46 ns |
| D38 | 7.89 ns | 8.02 ns | 14.8 ns | 12.6 ns | 13.1 ns |
| D57 | 6.27 ns | 7.17 ns | 7.16 ns | 8.09 ns | 8.09 ns |
| D76 | 8.73 ns | 8.71 ns | 8.71 ns | 9.51 ns | 9.51 ns |
| D115 | 14.4 ns | 14.1 ns | 12.5 ns | 12.4 ns | 14.1 ns |
| D153 | 12.8 ns | 16 ns | 20.1 ns | 8.84 ns | 20.1 ns |
| D230 | 32.2 ns | 36.3 ns | 36.2 ns | 32.6 ns | 31.8 ns |
| D307 | 41.3 ns | 48.5 ns | 30.2 ns | 44.3 ns | 43.1 ns |
| D462 | 88.1 ns | 96 ns | 73.6 ns | 64.9 ns | 71.2 ns |
| D616 | 102 ns | 99.8 ns | 96 ns | 77.6 ns | 83.8 ns |
| D924 | 109 ns | 104 ns | 99.8 ns | 109 ns | 93.9 ns |
| D1232 | 147 ns | 94.3 ns | 138 ns | 114 ns | 112 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,150.2 124.4,156.8 160.5,147.3 196.7,132.8 232.9,136.1 269.1,109.5 305.3,102.2 341.5,80.3 377.6,76.1 413.8,74.3 450.0,65.5 450.0,73.4 413.8,78.5 377.6,81.8 341.5,86.5 305.3,101.0 269.1,109.8 232.9,123.2 196.7,133.5 160.5,144.8 124.4,149.5 88.2,135.4 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,150.2 124.4,156.8 160.5,147.3 196.7,132.8 232.9,136.1 269.1,109.5 305.3,102.2 341.5,80.3 377.6,76.1 413.8,74.3 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,149.7 124.4,153.0 160.5,147.3 196.7,133.5 232.9,129.8 269.1,106.0 305.3,97.6 341.5,77.9 377.6,76.7 413.8,75.6 450.0,78.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,132.0 124.4,153.0 160.5,147.3 196.7,137.0 232.9,123.2 269.1,106.1 305.3,111.3 341.5,85.5 377.6,77.8 413.8,76.7 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,136.6 124.4,149.5 160.5,144.8 196.7,137.0 232.9,146.9 269.1,109.1 305.3,100.2 341.5,89.2 377.6,84.0 413.8,74.1 450.0,72.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,135.4 124.4,149.5 160.5,144.8 196.7,133.5 232.9,123.2 269.1,109.8 305.3,101.0 341.5,86.5 377.6,81.8 413.8,78.5 450.0,73.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 1.05 ns | 0.935 ns | 0.933 ns | 1.06 ns |
| D38 | 1.82 ns | 1.82 ns | 1.82 ns | 1.62 ns | 1.62 ns |
| D57 | 1.94 ns | 2.25 ns | 2.25 ns | 2.5 ns | 2.49 ns |
| D76 | 3.09 ns | 3.1 ns | 3.08 ns | 3.46 ns | 3.46 ns |
| D115 | 5.54 ns | 5.56 ns | 4.85 ns | 4.84 ns | 5.55 ns |
| D153 | 6.36 ns | 7.65 ns | 8.46 ns | 4.39 ns | 8.45 ns |
| D230 | 16.1 ns | 17.6 ns | 17.7 ns | 16.1 ns | 16.1 ns |
| D307 | 23.5 ns | 25.2 ns | 18 ns | 23.5 ns | 23.5 ns |
| D462 | 43.3 ns | 40.8 ns | 37.1 ns | 32.5 ns | 37 ns |
| D616 | 49.9 ns | 60.9 ns | 60.3 ns | 45.9 ns | 57.3 ns |
| D924 | 75.9 ns | 76.6 ns | 77.4 ns | 97 ns | 74.8 ns |
| D1232 | 106 ns | 59 ns | 106 ns | 95.3 ns | 94.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.5 88.2,147.0 124.4,145.6 160.5,135.5 196.7,122.8 232.9,119.8 269.1,99.7 305.3,91.4 341.5,78.2 377.6,75.1 413.8,66.0 450.0,58.7 450.0,61.2 413.8,66.3 377.6,72.1 341.5,81.6 305.3,91.5 269.1,99.6 232.9,113.6 196.7,122.8 160.5,133.1 124.4,140.2 88.2,149.6 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,147.0 124.4,145.6 160.5,135.5 196.7,122.8 232.9,119.8 269.1,99.7 305.3,91.4 341.5,78.2 377.6,75.1 413.8,66.0 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,146.9 124.4,142.4 160.5,135.5 196.7,122.8 232.9,115.8 269.1,97.7 305.3,89.9 341.5,79.5 377.6,70.8 413.8,65.8 450.0,71.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.9 124.4,142.4 160.5,135.5 196.7,125.7 232.9,113.6 269.1,97.6 305.3,97.2 341.5,81.5 377.6,71.0 413.8,65.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,133.0 196.7,125.7 232.9,127.9 269.1,99.7 305.3,91.5 341.5,84.4 377.6,76.9 413.8,60.7 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,140.2 160.5,133.1 196.7,122.8 232.9,113.6 269.1,99.6 305.3,91.5 341.5,81.6 377.6,72.1 413.8,66.3 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
