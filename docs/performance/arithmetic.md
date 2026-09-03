# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 ns | 0.985 ns | 0.82 ns | 1.25 ns | 1.25 ns |
| D38 | 1.61 ns | 1.61 ns | 1.61 ns | 1.82 ns | 1.62 ns |
| D57 | 2.25 ns | 1.61 ns | 2.26 ns | 2.25 ns | 2.25 ns |
| D76 | 1.56 ns | 3.47 ns | 3.08 ns | 2.71 ns | 1.79 ns |
| D115 | 4.42 ns | 4.39 ns | 4.42 ns | 4.4 ns | 4.39 ns |
| D153 | 5.49 ns | 6.64 ns | 6.62 ns | 3.15 ns | 6.64 ns |
| D230 | 11.8 ns | 11.9 ns | 14 ns | 13.9 ns | 15.4 ns |
| D307 | 19.6 ns | 13.5 ns | 16.3 ns | 18.5 ns | 18.5 ns |
| D462 | 28.6 ns | 28.6 ns | 33.4 ns | 32.6 ns | 32.7 ns |
| D616 | 45.1 ns | 74.6 ns | 57.2 ns | 50.8 ns | 39.5 ns |
| D924 | 75.8 ns | 74.7 ns | 85.2 ns | 51.3 ns | 84.9 ns |
| D1232 | 108 ns | 95.2 ns | 95.2 ns | 95 ns | 94.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,155.2 88.2,149.7 124.4,142.4 160.5,150.4 196.7,127.7 232.9,123.0 269.1,106.5 305.3,95.4 341.5,87.2 377.6,77.3 413.8,66.0 450.0,58.4 450.0,61.1 413.8,63.6 377.6,80.2 341.5,84.3 305.3,96.6 269.1,100.7 232.9,118.9 196.7,127.9 160.5,147.4 124.4,142.4 88.2,149.6 52.0,155.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,155.2 88.2,149.7 124.4,142.4 160.5,150.4 196.7,127.7 232.9,123.0 269.1,106.5 305.3,95.4 341.5,87.2 377.6,77.3 413.8,66.0 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.3 88.2,149.6 124.4,149.6 160.5,133.0 196.7,127.9 232.9,118.9 269.1,106.2 305.3,103.5 341.5,87.2 377.6,66.4 413.8,66.3 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,149.6 124.4,142.3 160.5,135.6 196.7,127.7 232.9,118.9 269.1,102.7 305.3,99.4 341.5,83.8 377.6,72.1 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,147.0 124.4,142.4 160.5,138.4 196.7,127.8 232.9,135.1 269.1,102.8 305.3,96.6 341.5,84.4 377.6,74.7 413.8,74.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,149.6 124.4,142.4 160.5,147.4 196.7,127.9 232.9,118.9 269.1,100.7 305.3,96.6 341.5,84.3 377.6,80.2 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.98 ns | 4.72 ns | 5.45 ns | 7.86 ns | 7.84 ns |
| D38 | 9.77 ns | 11.4 ns | 15.1 ns | 64 ns | 59.7 ns |
| D57 | 37.4 ns | 36.3 ns | 67 ns | 105 ns | 106 ns |
| D76 | 25.6 ns | 69.4 ns | 76.6 ns | 96.5 ns | 104 ns |
| D115 | 56.2 ns | 82.9 ns | 105 ns | 178 ns | 225 ns |
| D153 | 63 ns | 116 ns | 162 ns | 142 ns | 336 ns |
| D230 | 59.2 ns | 130 ns | 229 ns | 385 ns | 607 ns |
| D307 | 158 ns | 150 ns | 305 ns | 584 ns | 870 ns |
| D462 | 214 ns | 399 ns | 731 ns | 1.15 µs | 1.46 µs |
| D616 | 266 ns | 647 ns | 1.02 µs | 1.57 µs | 1.82 µs |
| D924 | 402 ns | 1.02 µs | 2.28 µs | 2.02 µs | 4.7 µs |
| D1232 | 670 ns | 1.75 µs | 3.5 µs | 4.41 µs | 7.43 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.2 88.2,160.5 124.4,131.4 160.5,139.6 196.7,122.5 232.9,120.0 269.1,121.4 305.3,100.1 341.5,93.4 377.6,88.8 413.8,79.8 450.0,68.7 450.0,16.4 413.8,26.4 377.6,47.0 341.5,51.8 305.3,63.0 269.1,70.8 232.9,83.7 196.7,92.4 160.5,109.1 124.4,108.7 88.2,121.2 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,160.5 124.4,131.4 160.5,139.6 196.7,122.5 232.9,120.0 269.1,121.4 305.3,100.1 341.5,93.4 377.6,88.8 413.8,79.8 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.3 88.2,157.1 124.4,132.0 160.5,117.9 196.7,114.1 232.9,106.7 269.1,104.3 305.3,101.2 341.5,79.9 377.6,69.4 413.8,59.6 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.2 88.2,151.0 124.4,118.7 160.5,115.8 196.7,108.8 232.9,99.6 269.1,92.0 305.3,85.8 341.5,66.8 377.6,59.5 413.8,42.1 450.0,32.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.2 88.2,119.7 124.4,108.9 160.5,110.8 196.7,97.5 232.9,102.3 269.1,80.7 305.3,71.7 341.5,56.9 377.6,50.2 413.8,44.7 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,121.2 124.4,108.7 160.5,109.1 196.7,92.4 232.9,83.7 269.1,70.8 305.3,63.0 341.5,51.8 377.6,47.0 413.8,26.4 450.0,16.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.886 ns | 2.09 ns | 2.48 ns | 4.93 ns | 5.12 ns |
| D38 | 3.5 ns | 15 ns | 25.3 ns | 30.3 ns | 26.3 ns |
| D57 | 6.24 ns | 17.3 ns | 33.6 ns | 71.3 ns | 71.9 ns |
| D76 | 3.96 ns | 38 ns | 42.9 ns | 65.2 ns | 73.5 ns |
| D115 | 13.4 ns | 47.1 ns | 93.4 ns | 194 ns | 227 ns |
| D153 | 17 ns | 56.4 ns | 121 ns | 135 ns | 398 ns |
| D230 | 18.1 ns | 99.2 ns | 338 ns | 518 ns | 1.02 µs |
| D307 | 54.4 ns | 111 ns | 392 ns | 1.03 µs | 1.38 µs |
| D462 | 81.2 ns | 408 ns | 1.31 µs | 1.83 µs | 2.6 µs |
| D616 | 92.4 ns | 731 ns | 1.72 µs | 2.29 µs | 2.94 µs |
| D924 | 138 ns | 1.24 µs | 3.16 µs | 3.5 µs | 8.2 µs |
| D1232 | 199 ns | 2.2 µs | 4.6 µs | 8.14 µs | 12.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,150.2 160.5,156.7 196.7,139.1 232.9,135.6 269.1,134.8 305.3,118.8 341.5,113.0 377.6,111.2 413.8,105.3 450.0,100.0 450.0,39.8 413.8,46.2 377.6,61.1 341.5,62.8 305.3,72.0 269.1,76.4 232.9,90.0 196.7,98.1 160.5,114.5 124.4,114.8 88.2,129.3 52.0,153.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,150.2 160.5,156.7 196.7,139.1 232.9,135.6 269.1,134.8 305.3,118.8 341.5,113.0 377.6,111.2 413.8,105.3 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,166.0 88.2,137.4 124.4,135.4 160.5,124.0 196.7,120.9 232.9,118.3 269.1,110.1 305.3,108.5 341.5,89.6 377.6,81.2 413.8,73.5 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.5 88.2,129.9 124.4,125.8 160.5,122.2 196.7,111.0 232.9,107.3 269.1,92.4 305.3,90.2 341.5,72.8 377.6,68.8 413.8,60.0 450.0,54.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,127.3 124.4,114.9 160.5,116.2 196.7,100.4 232.9,105.7 269.1,86.2 305.3,76.2 341.5,67.9 377.6,64.7 413.8,58.5 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.0 88.2,129.3 124.4,114.8 160.5,114.5 196.7,98.1 232.9,90.0 269.1,76.4 305.3,72.0 341.5,62.8 377.6,61.1 413.8,46.2 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.318 ns | 0.545 ns | 0.622 ns | 0.622 ns |
| D38 | 1.32 ns | 1.33 ns | 1.32 ns | 1.45 ns | 1.33 ns |
| D57 | 1.68 ns | 1.46 ns | 1.68 ns | 1.68 ns | 1.68 ns |
| D76 | 1.39 ns | 2.17 ns | 2.1 ns | 1.68 ns | 1.73 ns |
| D115 | 2.83 ns | 2.83 ns | 2.83 ns | 3.28 ns | 3.28 ns |
| D153 | 3.56 ns | 4.22 ns | 4.6 ns | 2.32 ns | 4.6 ns |
| D230 | 4.19 ns | 5.16 ns | 7.17 ns | 7.17 ns | 7.24 ns |
| D307 | 12.3 ns | 6.41 ns | 7.62 ns | 11.1 ns | 11.1 ns |
| D462 | 15 ns | 14.9 ns | 17 ns | 16.7 ns | 16.7 ns |
| D616 | 18.9 ns | 21.9 ns | 27.8 ns | 18.1 ns | 18 ns |
| D924 | 56 ns | 81.6 ns | 85 ns | 47.5 ns | 84.7 ns |
| D1232 | 54.9 ns | 61.8 ns | 61.5 ns | 61.9 ns | 61.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.2 124.4,128.3 160.5,133.8 196.7,113.2 232.9,106.6 269.1,101.9 305.3,70.8 341.5,64.9 377.6,58.2 413.8,26.8 450.0,27.4 450.0,24.0 413.8,14.8 377.6,59.7 341.5,61.9 305.3,73.7 269.1,86.0 232.9,99.2 196.7,108.9 160.5,127.5 124.4,128.3 88.2,135.1 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.2 124.4,128.3 160.5,133.8 196.7,113.2 232.9,106.6 269.1,101.9 305.3,70.8 341.5,64.9 377.6,58.2 413.8,26.8 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.5 88.2,135.2 124.4,132.4 160.5,121.0 196.7,113.2 232.9,101.6 269.1,95.8 305.3,89.6 341.5,65.1 377.6,53.9 413.8,15.9 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,135.2 124.4,128.3 160.5,121.9 196.7,113.2 232.9,99.2 269.1,86.3 305.3,84.5 341.5,61.3 377.6,47.1 413.8,14.7 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,128.3 160.5,128.3 196.7,108.9 232.9,119.0 269.1,86.3 305.3,73.7 341.5,61.8 377.6,59.5 413.8,31.6 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,127.5 196.7,108.9 232.9,99.2 269.1,86.0 305.3,73.7 341.5,61.9 377.6,59.7 413.8,14.8 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.69 ns | 1.64 ns | 2.18 ns | 2.18 ns |
| D38 | 7.15 ns | 7.15 ns | 12 ns | 15 ns | 13.1 ns |
| D57 | 7.17 ns | 5.07 ns | 7.16 ns | 7.17 ns | 7.16 ns |
| D76 | 4.54 ns | 9.5 ns | 8.51 ns | 7.37 ns | 4.74 ns |
| D115 | 12.7 ns | 12.4 ns | 12.4 ns | 12.4 ns | 12.5 ns |
| D153 | 15.8 ns | 20 ns | 20 ns | 8.73 ns | 20 ns |
| D230 | 19.6 ns | 28.1 ns | 32.1 ns | 32 ns | 36.4 ns |
| D307 | 48.1 ns | 27.4 ns | 31.9 ns | 42.6 ns | 42.5 ns |
| D462 | 83.8 ns | 84.4 ns | 90.7 ns | 86.5 ns | 83.3 ns |
| D616 | 84 ns | 101 ns | 86.4 ns | 73.1 ns | 62.9 ns |
| D924 | 122 ns | 137 ns | 111 ns | 59.4 ns | 110 ns |
| D1232 | 147 ns | 134 ns | 127 ns | 116 ns | 115 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,153.0 124.4,153.0 160.5,166.2 196.7,136.5 232.9,130.1 269.1,123.8 305.3,97.9 341.5,81.8 377.6,81.7 413.8,71.0 450.0,65.4 450.0,72.6 413.8,73.9 377.6,90.1 341.5,82.0 305.3,101.4 269.1,105.9 232.9,123.2 196.7,137.0 160.5,165.0 124.4,153.0 88.2,135.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,153.0 124.4,153.0 160.5,166.2 196.7,136.5 232.9,130.1 269.1,123.8 305.3,97.9 341.5,81.8 377.6,81.7 413.8,71.0 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,194.9 88.2,153.1 124.4,163.0 160.5,144.8 196.7,137.0 232.9,123.2 269.1,113.4 305.3,114.1 341.5,81.6 377.6,76.3 413.8,67.6 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.8 88.2,137.9 124.4,153.0 160.5,148.0 196.7,137.0 232.9,123.2 269.1,109.6 305.3,109.8 341.5,79.5 377.6,80.9 413.8,73.7 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,131.5 124.4,153.0 160.5,152.2 196.7,137.0 232.9,147.3 269.1,109.6 305.3,101.4 341.5,80.9 377.6,85.7 413.8,91.8 450.0,72.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,135.5 124.4,153.0 160.5,165.0 196.7,137.0 232.9,123.2 269.1,105.9 305.3,101.4 341.5,82.0 377.6,90.1 413.8,73.9 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.938 ns | 0.989 ns | 0.819 ns | 0.935 ns | 0.939 ns |
| D38 | 1.6 ns | 1.61 ns | 1.61 ns | 1.81 ns | 1.61 ns |
| D57 | 2.26 ns | 1.78 ns | 2.25 ns | 2.25 ns | 2.25 ns |
| D76 | 1.83 ns | 3.45 ns | 3.08 ns | 2.68 ns | 2.09 ns |
| D115 | 4.9 ns | 4.88 ns | 4.88 ns | 4.88 ns | 4.89 ns |
| D153 | 7.02 ns | 8.44 ns | 8.44 ns | 4.35 ns | 8.45 ns |
| D230 | 12.9 ns | 13.8 ns | 16.1 ns | 16.1 ns | 17.7 ns |
| D307 | 25.1 ns | 15.7 ns | 18.6 ns | 23.4 ns | 23.4 ns |
| D462 | 36.7 ns | 36.8 ns | 43.3 ns | 42.3 ns | 42.3 ns |
| D616 | 45.5 ns | 71.8 ns | 59.4 ns | 50.2 ns | 43.3 ns |
| D924 | 75.5 ns | 74.4 ns | 85.3 ns | 56.6 ns | 85 ns |
| D1232 | 108 ns | 96.1 ns | 96.5 ns | 95.3 ns | 95.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.8 124.4,142.3 160.5,146.9 196.7,125.5 232.9,117.7 269.1,104.4 305.3,90.0 341.5,81.8 377.6,77.1 413.8,66.1 450.0,58.4 450.0,61.0 413.8,63.5 377.6,78.2 341.5,78.7 305.3,91.5 269.1,97.6 232.9,113.6 196.7,125.5 160.5,144.0 124.4,142.4 88.2,149.7 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.8 124.4,142.3 160.5,146.9 196.7,125.5 232.9,117.7 269.1,104.4 305.3,90.0 341.5,81.8 377.6,77.1 413.8,66.1 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.2 88.2,149.7 124.4,147.5 160.5,133.1 196.7,125.6 232.9,113.7 269.1,103.1 305.3,100.2 341.5,81.7 377.6,67.2 413.8,66.4 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,149.7 124.4,142.4 160.5,135.6 196.7,125.6 232.9,113.7 269.1,99.6 305.3,96.5 341.5,78.2 377.6,71.3 413.8,63.5 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,147.1 124.4,142.4 160.5,138.6 196.7,125.6 232.9,128.1 269.1,99.7 305.3,91.5 341.5,78.7 377.6,75.0 413.8,72.4 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,142.4 160.5,144.0 196.7,125.5 232.9,113.6 269.1,97.6 305.3,91.5 341.5,78.7 377.6,78.2 413.8,63.5 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
