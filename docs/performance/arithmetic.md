# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.16 ns | 0.937 ns | 0.98 ns | 0.936 ns |
| D38 | 1.25 ns | 1.61 ns | 1.61 ns | 1.6 ns | 1.41 ns |
| D57 | 2.5 ns | 2.28 ns | 1.94 ns | 2.5 ns | 2.5 ns |
| D76 | 3.49 ns | 1.82 ns | 1.79 ns | 2.08 ns | 3.08 ns |
| D115 | 5.01 ns | 4.4 ns | 4.99 ns | 3.86 ns | 3.08 ns |
| D153 | 5.9 ns | 3.94 ns | 5.9 ns | 5.89 ns | 5.9 ns |
| D230 | 15.3 ns | 13.9 ns | 15.3 ns | 15.3 ns | 10.2 ns |
| D307 | 18.7 ns | 18.7 ns | 18.7 ns | 18.7 ns | 18.7 ns |
| D462 | 33.2 ns | 29.1 ns | 32.6 ns | 49.9 ns | 32.6 ns |
| D616 | 45 ns | 70.1 ns | 72.3 ns | 49.6 ns | 63 ns |
| D924 | 85.2 ns | 83 ns | 74.6 ns | 74.8 ns | 74.9 ns |
| D1232 | 95 ns | 105 ns | 76.9 ns | 95.2 ns | 70.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,155.1 124.4,140.1 160.5,132.9 196.7,125.0 232.9,121.5 269.1,100.7 305.3,96.4 341.5,83.9 377.6,77.3 413.8,63.5 450.0,61.1 450.0,67.5 413.8,66.3 377.6,70.0 341.5,84.4 305.3,96.4 269.1,109.6 232.9,121.5 196.7,135.6 160.5,135.6 124.4,140.1 88.2,152.5 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,155.1 124.4,140.1 160.5,132.9 196.7,125.0 232.9,121.5 269.1,100.7 305.3,96.4 341.5,83.9 377.6,77.3 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,156.8 88.2,149.7 124.4,142.1 160.5,147.0 196.7,127.8 232.9,130.2 269.1,102.9 305.3,96.4 341.5,86.8 377.6,67.7 413.8,64.0 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,145.7 160.5,147.4 196.7,125.1 232.9,121.5 269.1,100.7 305.3,96.4 341.5,84.3 377.6,67.1 413.8,66.4 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.4 88.2,149.7 124.4,140.1 160.5,144.1 196.7,130.6 232.9,121.5 269.1,100.8 305.3,96.4 341.5,75.1 377.6,75.2 413.8,66.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,152.5 124.4,140.1 160.5,135.6 196.7,135.6 232.9,121.5 269.1,109.6 305.3,96.4 341.5,84.4 377.6,70.0 413.8,66.3 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.44 ns | 5.59 ns | 5.51 ns | 7.21 ns | 7.84 ns |
| D38 | 8.83 ns | 11.3 ns | 15.4 ns | 726 ns | 823 ns |
| D57 | 35.4 ns | 48.1 ns | 57.2 ns | 130 ns | 113 ns |
| D76 | 40.4 ns | 39.9 ns | 51.7 ns | 88.3 ns | 132 ns |
| D115 | 58.6 ns | 83 ns | 110 ns | 150 ns | 179 ns |
| D153 | 67.6 ns | 72.5 ns | 144 ns | 228 ns | 306 ns |
| D230 | 109 ns | 152 ns | 255 ns | 423 ns | 385 ns |
| D307 | 132 ns | 221 ns | 362 ns | 582 ns | 873 ns |
| D462 | 237 ns | 397 ns | 740 ns | 1.05 µs | 1.45 µs |
| D616 | 262 ns | 652 ns | 1.1 µs | 1.55 µs | 2.4 µs |
| D924 | 431 ns | 1.11 µs | 2.08 µs | 2.61 µs | 4.37 µs |
| D1232 | 532 ns | 1.78 µs | 2.83 µs | 4.37 µs | 5.96 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.6 88.2,162.7 124.4,132.5 160.5,129.7 196.7,121.6 232.9,118.5 269.1,108.1 305.3,104.0 341.5,91.3 377.6,89.1 413.8,78.3 450.0,73.7 450.0,21.2 413.8,28.0 377.6,41.0 341.5,51.9 305.3,63.0 269.1,80.7 232.9,85.7 196.7,97.4 160.5,103.9 124.4,107.4 88.2,64.2 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.6 88.2,162.7 124.4,132.5 160.5,129.7 196.7,121.6 232.9,118.5 269.1,108.1 305.3,104.0 341.5,91.3 377.6,89.1 413.8,78.3 450.0,73.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,172.6 88.2,157.3 124.4,125.9 160.5,130.0 196.7,114.1 232.9,117.0 269.1,100.9 305.3,92.8 341.5,80.1 377.6,69.3 413.8,57.7 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.9 88.2,150.7 124.4,122.1 160.5,124.3 196.7,107.9 232.9,102.2 269.1,89.7 305.3,82.1 341.5,66.6 377.6,57.9 413.8,44.1 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.1 88.2,66.9 124.4,104.3 160.5,112.7 196.7,101.2 232.9,92.1 269.1,78.7 305.3,71.8 341.5,59.0 377.6,50.5 413.8,39.2 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,64.2 124.4,107.4 160.5,103.9 196.7,97.4 232.9,85.7 269.1,80.7 305.3,63.0 341.5,51.9 377.6,41.0 413.8,28.0 450.0,21.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 2.46 ns | 3.02 ns | 3.63 ns | 5.12 ns |
| D38 | 3.43 ns | 14.9 ns | 27.1 ns | 25.1 ns | 27.5 ns |
| D57 | 4.23 ns | 21.5 ns | 27.3 ns | 77.6 ns | 77.4 ns |
| D76 | 5.7 ns | 25.4 ns | 30.9 ns | 65.4 ns | 101 ns |
| D115 | 13.7 ns | 47 ns | 93.1 ns | 166 ns | 180 ns |
| D153 | 18.4 ns | 31.5 ns | 112 ns | 236 ns | 355 ns |
| D230 | 27.6 ns | 123 ns | 368 ns | 568 ns | 590 ns |
| D307 | 43.8 ns | 172 ns | 461 ns | 1.02 µs | 1.37 µs |
| D462 | 102 ns | 406 ns | 1.3 µs | 1.77 µs | 2.59 µs |
| D616 | 90.6 ns | 723 ns | 1.83 µs | 2.28 µs | 4.16 µs |
| D924 | 163 ns | 1.48 µs | 3 µs | 4.97 µs | 7.74 µs |
| D1232 | 179 ns | 2.23 µs | 4.11 µs | 8.3 µs | 10 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,188.6 124.4,184.9 160.5,179.8 196.7,164.5 232.9,159.4 269.1,152.4 305.3,144.3 341.5,129.6 377.6,131.7 413.8,121.5 450.0,119.9 450.0,49.9 413.8,54.5 377.6,65.2 341.5,73.5 305.3,84.5 269.1,99.2 232.9,108.0 196.7,119.7 160.5,129.8 124.4,134.4 88.2,152.4 52.0,181.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,188.6 124.4,184.9 160.5,179.8 196.7,164.5 232.9,159.4 269.1,152.4 305.3,144.3 341.5,129.6 377.6,131.7 413.8,121.5 450.0,119.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,194.3 88.2,163.1 124.4,156.7 160.5,153.8 196.7,143.1 232.9,150.1 269.1,126.5 305.3,120.5 341.5,105.6 377.6,95.6 413.8,83.2 450.0,76.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.8 88.2,152.7 124.4,152.6 160.5,150.4 196.7,131.2 232.9,128.1 269.1,107.4 305.3,103.4 341.5,85.4 377.6,79.5 413.8,70.9 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,154.0 124.4,134.4 160.5,137.4 196.7,121.2 232.9,115.1 269.1,99.8 305.3,89.7 341.5,80.1 377.6,75.7 413.8,62.1 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.6 88.2,152.4 124.4,134.4 160.5,129.8 196.7,119.7 232.9,108.0 269.1,99.2 305.3,84.5 341.5,73.5 377.6,65.2 413.8,54.5 450.0,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.704 ns | 0.66 ns | 0.623 ns | 0.493 ns | 0.622 ns |
| D38 | 1.12 ns | 1.41 ns | 1.41 ns | 1.41 ns | 1.12 ns |
| D57 | 1.74 ns | 1.87 ns | 1.35 ns | 1.74 ns | 1.74 ns |
| D76 | 2.16 ns | 1.49 ns | 1.47 ns | 1.68 ns | 2.49 ns |
| D115 | 3.17 ns | 2.83 ns | 3.17 ns | 2.75 ns | 2.39 ns |
| D153 | 3.79 ns | 2.64 ns | 4.29 ns | 4.29 ns | 4.29 ns |
| D230 | 6.65 ns | 6 ns | 7.44 ns | 7.43 ns | 4.22 ns |
| D307 | 10.9 ns | 11.1 ns | 11.1 ns | 11.1 ns | 11.1 ns |
| D462 | 17.5 ns | 14.9 ns | 16.7 ns | 31.2 ns | 16.6 ns |
| D616 | 19 ns | 21.8 ns | 21.8 ns | 17.3 ns | 21.7 ns |
| D924 | 63.2 ns | 81.3 ns | 75.7 ns | 76.7 ns | 75.6 ns |
| D1232 | 47.1 ns | 65.4 ns | 51.3 ns | 61.5 ns | 42.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,140.1 124.4,127.3 160.5,121.0 196.7,110.0 232.9,104.8 269.1,88.5 305.3,74.2 341.5,60.4 377.6,58.1 413.8,23.3 450.0,31.8 450.0,35.0 413.8,18.1 377.6,54.3 341.5,61.9 305.3,73.7 269.1,101.6 232.9,101.2 196.7,118.1 160.5,116.9 124.4,127.3 88.2,140.0 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,140.1 124.4,127.3 160.5,121.0 196.7,110.0 232.9,104.8 269.1,88.5 305.3,74.2 341.5,60.4 377.6,58.1 413.8,23.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.4 88.2,133.3 124.4,125.2 160.5,131.9 196.7,113.3 232.9,115.2 269.1,91.5 305.3,73.7 341.5,65.1 377.6,54.1 413.8,16.0 450.0,22.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,133.3 124.4,134.7 160.5,132.1 196.7,110.0 232.9,101.1 269.1,85.2 305.3,73.7 341.5,61.8 377.6,54.1 413.8,18.0 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.8 88.2,133.3 124.4,127.3 160.5,128.2 196.7,114.1 232.9,101.2 269.1,85.3 305.3,73.7 341.5,43.7 377.6,60.8 413.8,17.7 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,140.0 124.4,127.3 160.5,116.9 196.7,118.1 232.9,101.2 269.1,101.6 305.3,73.7 341.5,61.9 377.6,54.3 413.8,18.1 450.0,35.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.01 ns | 1.87 ns | 2.79 ns | 2.18 ns |
| D38 | 6.05 ns | 6.95 ns | 12.3 ns | 13 ns | 12.7 ns |
| D57 | 8.09 ns | 7.16 ns | 6.27 ns | 8.09 ns | 8.09 ns |
| D76 | 9.51 ns | 5.36 ns | 4.79 ns | 5.56 ns | 8.46 ns |
| D115 | 14.4 ns | 12.4 ns | 14.1 ns | 10.9 ns | 7.57 ns |
| D153 | 17.2 ns | 10.2 ns | 16.1 ns | 16.2 ns | 16.6 ns |
| D230 | 36.7 ns | 31.9 ns | 36.2 ns | 36 ns | 17.3 ns |
| D307 | 41.3 ns | 40.6 ns | 42.9 ns | 42.6 ns | 42.5 ns |
| D462 | 91.3 ns | 75.3 ns | 89.6 ns | 93 ns | 84.1 ns |
| D616 | 82.3 ns | 102 ns | 110 ns | 71.3 ns | 92.8 ns |
| D924 | 112 ns | 107 ns | 98.9 ns | 93.5 ns | 98.6 ns |
| D1232 | 132 ns | 141 ns | 92 ns | 118 ns | 81.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,157.9 124.4,149.5 160.5,144.8 196.7,132.8 232.9,127.7 269.1,105.7 305.3,102.3 341.5,79.3 377.6,82.3 413.8,73.3 450.0,68.6 450.0,82.5 413.8,77.1 377.6,78.8 341.5,81.7 305.3,101.4 269.1,127.5 232.9,128.6 196.7,151.4 160.5,148.2 124.4,149.5 88.2,136.4 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,157.9 124.4,149.5 160.5,144.8 196.7,132.8 232.9,127.7 269.1,105.7 305.3,102.3 341.5,79.3 377.6,82.3 413.8,73.3 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,189.7 88.2,153.9 124.4,153.0 160.5,161.4 196.7,137.1 232.9,142.8 269.1,109.7 305.3,102.7 341.5,84.9 377.6,76.1 413.8,74.7 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,137.4 124.4,156.8 160.5,164.6 196.7,133.5 232.9,129.5 269.1,106.1 305.3,101.1 341.5,79.9 377.6,74.0 413.8,77.0 450.0,79.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.2 88.2,135.7 124.4,149.5 160.5,160.3 196.7,140.8 232.9,129.3 269.1,106.3 305.3,101.4 341.5,78.8 377.6,86.5 413.8,78.6 450.0,71.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,136.4 124.4,149.5 160.5,148.2 196.7,151.4 232.9,128.6 269.1,127.5 305.3,101.4 341.5,81.7 377.6,78.8 413.8,77.1 450.0,82.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.18 ns | 0.936 ns | 0.981 ns | 0.935 ns |
| D38 | 1.13 ns | 1.61 ns | 1.61 ns | 1.62 ns | 1.41 ns |
| D57 | 2.51 ns | 2.27 ns | 1.94 ns | 2.51 ns | 2.5 ns |
| D76 | 3.46 ns | 2.13 ns | 2.09 ns | 2.44 ns | 3.1 ns |
| D115 | 5.54 ns | 4.89 ns | 5.53 ns | 4.31 ns | 3.66 ns |
| D153 | 7.64 ns | 5.22 ns | 7.67 ns | 7.62 ns | 7.63 ns |
| D230 | 17.7 ns | 16.2 ns | 17.6 ns | 17.6 ns | 11.2 ns |
| D307 | 23.5 ns | 23.5 ns | 23.5 ns | 23.5 ns | 23.5 ns |
| D462 | 43.2 ns | 37.3 ns | 42.4 ns | 53.7 ns | 42.3 ns |
| D616 | 45.2 ns | 75.3 ns | 79.1 ns | 50 ns | 61.6 ns |
| D924 | 84.8 ns | 84.5 ns | 75.1 ns | 75.1 ns | 74.9 ns |
| D1232 | 95.4 ns | 105 ns | 86.9 ns | 95 ns | 79.1 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,157.4 124.4,140.0 160.5,133.0 196.7,122.8 232.9,115.8 269.1,97.7 305.3,91.5 341.5,78.2 377.6,77.2 413.8,63.6 450.0,61.0 450.0,65.1 413.8,66.3 377.6,70.5 341.5,78.7 305.3,91.4 269.1,107.5 232.9,115.9 196.7,131.8 160.5,135.5 124.4,140.1 88.2,152.6 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,157.4 124.4,140.0 160.5,133.0 196.7,122.8 232.9,115.8 269.1,97.7 305.3,91.5 341.5,78.2 377.6,77.2 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,156.5 88.2,149.6 124.4,142.2 160.5,143.6 196.7,125.6 232.9,124.1 269.1,99.5 305.3,91.4 341.5,81.4 377.6,66.2 413.8,63.6 450.0,58.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,145.7 160.5,144.0 196.7,122.8 232.9,115.7 269.1,97.7 305.3,91.5 341.5,78.7 377.6,65.1 413.8,66.2 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.4 88.2,149.6 124.4,140.0 160.5,140.6 196.7,128.3 232.9,115.9 269.1,97.7 305.3,91.4 341.5,73.5 377.6,75.0 413.8,66.2 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,152.6 124.4,140.1 160.5,135.5 196.7,131.8 232.9,115.9 269.1,107.5 305.3,91.4 341.5,78.7 377.6,70.5 413.8,66.3 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
