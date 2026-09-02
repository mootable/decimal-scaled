# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.98 ns | 0.991 ns | 0.937 ns | 0.819 ns | 0.967 ns |
| D38 | 1.62 ns | 1.01 ns | 1.62 ns | 1.61 ns | 1.62 ns |
| D57 | 2.28 ns | 1.7 ns | 2.28 ns | 2.28 ns | 2.28 ns |
| D76 | 3.08 ns | 3.09 ns | 3.09 ns | 3.48 ns | 3.08 ns |
| D115 | 2.88 ns | 3.17 ns | 4.99 ns | 4.4 ns | 4.4 ns |
| D153 | 4.14 ns | 3.63 ns | 5.89 ns | 5.9 ns | 6.65 ns |
| D230 | 15.4 ns | 13.9 ns | 13.8 ns | 15.3 ns | 13.9 ns |
| D307 | 14.5 ns | 18.5 ns | 18.6 ns | 18.6 ns | 19.6 ns |
| D462 | 32.6 ns | 58 ns | 33.3 ns | 29.1 ns | 29.1 ns |
| D616 | 47.4 ns | 45.5 ns | 45.3 ns | 58.3 ns | 44.8 ns |
| D924 | 84.8 ns | 84.9 ns | 98.3 ns | 63 ns | 84.9 ns |
| D1232 | 120 ns | 106 ns | 106 ns | 82.1 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.4 88.2,149.5 124.4,142.1 160.5,135.6 196.7,137.0 232.9,129.2 269.1,100.7 305.3,101.9 341.5,84.3 377.6,76.2 413.8,63.6 450.0,56.0 450.0,58.6 413.8,63.5 377.6,77.5 341.5,86.8 305.3,95.3 269.1,102.8 232.9,118.9 196.7,127.8 160.5,135.5 124.4,142.1 88.2,149.6 52.0,160.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.4 88.2,149.5 124.4,142.1 160.5,135.6 196.7,137.0 232.9,129.2 269.1,100.7 305.3,101.9 341.5,84.3 377.6,76.2 413.8,63.6 450.0,56.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.2 88.2,159.8 124.4,148.4 160.5,135.5 196.7,135.0 232.9,132.0 269.1,102.8 305.3,96.6 341.5,71.8 377.6,77.1 413.8,63.6 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.1 160.5,135.5 196.7,125.1 232.9,121.5 269.1,102.9 305.3,96.6 341.5,83.9 377.6,77.2 413.8,60.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,149.6 124.4,142.1 160.5,132.9 196.7,127.8 232.9,121.5 269.1,100.7 305.3,96.6 341.5,86.8 377.6,71.7 413.8,70.0 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.7 88.2,149.6 124.4,142.1 160.5,135.5 196.7,127.8 232.9,118.9 269.1,102.8 305.3,95.3 341.5,86.8 377.6,77.5 413.8,63.5 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.88 ns | 4.71 ns | 5.61 ns | 6.83 ns | 7.99 ns |
| D38 | 9.89 ns | 8.6 ns | 15.6 ns | 58.6 ns | 59.5 ns |
| D57 | 38.1 ns | 36.5 ns | 68.9 ns | 106 ns | 107 ns |
| D76 | 39.6 ns | 61.3 ns | 76 ns | 110 ns | 132 ns |
| D115 | 37.7 ns | 63.6 ns | 112 ns | 176 ns | 220 ns |
| D153 | 52.9 ns | 64.5 ns | 144 ns | 232 ns | 334 ns |
| D230 | 110 ns | 152 ns | 228 ns | 424 ns | 562 ns |
| D307 | 85.4 ns | 223 ns | 367 ns | 575 ns | 952 ns |
| D462 | 252 ns | 457 ns | 582 ns | 1.07 µs | 1.38 µs |
| D616 | 231 ns | 621 ns | 1.03 µs | 1.82 µs | 2.25 µs |
| D924 | 427 ns | 1.22 µs | 2.29 µs | 2.46 µs | 4.71 µs |
| D1232 | 645 ns | 1.96 µs | 3.97 µs | 3.74 µs | 8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.6 88.2,160.2 124.4,130.9 160.5,130.1 196.7,131.2 232.9,123.8 269.1,108.0 305.3,113.4 341.5,89.9 377.6,91.8 413.8,78.5 450.0,69.5 450.0,14.8 413.8,26.4 377.6,42.4 341.5,53.0 305.3,61.1 269.1,72.5 232.9,83.8 196.7,92.9 160.5,103.9 124.4,108.6 88.2,121.3 52.0,164.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.6 88.2,160.2 124.4,130.9 160.5,130.1 196.7,131.2 232.9,123.8 269.1,108.0 305.3,113.4 341.5,89.9 377.6,91.8 413.8,78.5 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.3 88.2,163.3 124.4,131.9 160.5,120.6 196.7,119.8 232.9,119.5 269.1,100.9 305.3,92.6 341.5,77.0 377.6,70.3 413.8,55.6 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.6 88.2,150.4 124.4,118.1 160.5,116.0 196.7,107.6 232.9,102.1 269.1,92.1 305.3,81.8 341.5,71.8 377.6,59.4 413.8,42.0 450.0,30.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.3 88.2,121.6 124.4,108.7 160.5,107.9 196.7,97.7 232.9,91.7 269.1,78.7 305.3,72.0 341.5,58.5 377.6,47.0 413.8,40.5 450.0,31.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.9 88.2,121.3 124.4,108.6 160.5,103.9 196.7,92.9 232.9,83.8 269.1,72.5 305.3,61.1 341.5,53.0 377.6,42.4 413.8,26.4 450.0,14.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.649 ns | 2.09 ns | 3.13 ns | 3.9 ns | 5.14 ns |
| D38 | 3.5 ns | 11.2 ns | 26.2 ns | 27.5 ns | 27.4 ns |
| D57 | 6.23 ns | 16.9 ns | 33.4 ns | 71.6 ns | 71.5 ns |
| D76 | 7.79 ns | 35.1 ns | 42.5 ns | 83.6 ns | 101 ns |
| D115 | 10.7 ns | 32.8 ns | 93.1 ns | 197 ns | 229 ns |
| D153 | 12 ns | 28.6 ns | 112 ns | 241 ns | 397 ns |
| D230 | 27.7 ns | 122 ns | 340 ns | 566 ns | 988 ns |
| D307 | 27.2 ns | 166 ns | 453 ns | 1.02 µs | 1.44 µs |
| D462 | 87.3 ns | 450 ns | 1.04 µs | 1.72 µs | 2.43 µs |
| D616 | 86.2 ns | 655 ns | 1.73 µs | 2.69 µs | 3.85 µs |
| D924 | 162 ns | 1.57 µs | 3.17 µs | 4.44 µs | 8.2 µs |
| D1232 | 204 ns | 2.38 µs | 4.99 µs | 6.97 µs | 14 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.9 88.2,158.5 124.4,150.2 160.5,146.9 196.7,142.3 232.9,140.7 269.1,128.6 305.3,128.8 341.5,112.0 377.6,112.2 413.8,103.0 450.0,99.7 450.0,38.5 413.8,46.2 377.6,57.2 341.5,63.8 305.3,71.4 269.1,76.8 232.9,90.0 196.7,98.0 160.5,109.9 124.4,114.9 88.2,128.7 52.0,153.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.9 88.2,158.5 124.4,150.2 160.5,146.9 196.7,142.3 232.9,140.7 269.1,128.6 305.3,128.8 341.5,112.0 377.6,112.2 413.8,103.0 450.0,99.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,166.0 88.2,141.7 124.4,135.7 160.5,125.2 196.7,126.2 232.9,128.1 269.1,107.1 305.3,102.7 341.5,88.2 377.6,82.8 413.8,70.1 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,129.4 124.4,125.9 160.5,122.4 196.7,111.0 232.9,108.4 269.1,92.3 305.3,88.1 341.5,76.0 377.6,68.7 413.8,60.0 450.0,53.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,128.7 124.4,114.8 160.5,112.6 196.7,100.2 232.9,97.3 269.1,84.9 305.3,76.4 341.5,68.8 377.6,62.4 413.8,55.1 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.0 88.2,128.7 124.4,114.9 160.5,109.9 196.7,98.0 232.9,90.0 269.1,76.8 305.3,71.4 341.5,63.8 377.6,57.2 413.8,46.2 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.317 ns | 0.318 ns | 0.622 ns | 0.545 ns | 0.623 ns |
| D38 | 1.33 ns | 0.96 ns | 1.32 ns | 1.32 ns | 1.32 ns |
| D57 | 1.87 ns | 1.44 ns | 1.87 ns | 1.87 ns | 1.87 ns |
| D76 | 2.18 ns | 2.18 ns | 2.19 ns | 2.17 ns | 2.48 ns |
| D115 | 2.21 ns | 2.43 ns | 3.16 ns | 3.28 ns | 3.28 ns |
| D153 | 2.94 ns | 2.37 ns | 4.49 ns | 4.49 ns | 4.6 ns |
| D230 | 6.65 ns | 5.86 ns | 7.16 ns | 7.23 ns | 7.16 ns |
| D307 | 5.75 ns | 11.1 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 17.7 ns | 16.7 ns | 14 ns | 14.9 ns | 14.9 ns |
| D616 | 17.9 ns | 20 ns | 19.9 ns | 20 ns | 19.8 ns |
| D924 | 63.6 ns | 84.7 ns | 96.3 ns | 68.9 ns | 84.7 ns |
| D1232 | 77 ns | 69.8 ns | 69.8 ns | 44.6 ns | 69.7 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.6 88.2,135.2 124.4,125.2 160.5,120.7 196.7,120.3 232.9,112.2 269.1,88.5 305.3,92.7 341.5,60.1 377.6,59.9 413.8,23.1 450.0,17.6 450.0,20.5 413.8,14.8 377.6,56.9 341.5,65.1 305.3,70.2 269.1,86.4 232.9,99.1 196.7,108.9 160.5,117.0 124.4,125.2 88.2,135.2 52.0,157.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.6 88.2,135.2 124.4,125.2 160.5,120.7 196.7,120.3 232.9,112.2 269.1,88.5 305.3,92.7 341.5,60.1 377.6,59.9 413.8,23.1 450.0,17.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.5 88.2,144.5 124.4,132.8 160.5,120.7 196.7,117.6 232.9,118.4 269.1,92.2 305.3,73.8 341.5,61.9 377.6,56.7 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,120.7 196.7,110.0 232.9,99.9 269.1,86.4 305.3,73.7 341.5,67.0 377.6,56.7 413.8,11.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,135.2 124.4,125.2 160.5,121.0 196.7,108.9 232.9,99.8 269.1,86.0 305.3,73.7 341.5,65.1 377.6,56.7 413.8,20.8 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.0 88.2,135.2 124.4,125.2 160.5,117.0 196.7,108.9 232.9,99.1 269.1,86.4 305.3,70.2 341.5,65.1 377.6,56.9 413.8,14.8 450.0,20.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.68 ns | 1.69 ns | 1.87 ns | 1.91 ns | 2.18 ns |
| D38 | 7.16 ns | 5.19 ns | 12.1 ns | 12.5 ns | 13.1 ns |
| D57 | 7.16 ns | 4.78 ns | 7.17 ns | 7.16 ns | 7.16 ns |
| D76 | 8.42 ns | 8.44 ns | 8.72 ns | 9.84 ns | 8.67 ns |
| D115 | 7.94 ns | 8.78 ns | 14.1 ns | 12.4 ns | 12.4 ns |
| D153 | 11.5 ns | 9.12 ns | 16 ns | 16.3 ns | 20.1 ns |
| D230 | 36.8 ns | 32 ns | 32 ns | 36 ns | 32.1 ns |
| D307 | 28.1 ns | 43.2 ns | 53.7 ns | 42.6 ns | 47.8 ns |
| D462 | 89.7 ns | 100 ns | 77.6 ns | 77 ns | 70.3 ns |
| D616 | 75.3 ns | 81.1 ns | 79 ns | 86.5 ns | 77 ns |
| D924 | 114 ns | 113 ns | 111 ns | 72.2 ns | 94.1 ns |
| D1232 | 154 ns | 136 ns | 131 ns | 75.5 ns | 128 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,153.0 124.4,153.0 160.5,148.3 196.7,150.0 232.9,139.3 269.1,105.6 305.3,113.4 341.5,79.8 377.6,84.9 413.8,72.8 450.0,64.1 450.0,69.5 413.8,78.4 377.6,84.2 341.5,86.9 305.3,98.0 269.1,109.6 232.9,123.2 196.7,137.0 160.5,147.5 124.4,153.0 88.2,135.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,153.0 124.4,153.0 160.5,148.3 196.7,150.0 232.9,139.3 269.1,105.6 305.3,113.4 341.5,79.8 377.6,84.9 413.8,72.8 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,194.7 88.2,162.3 124.4,164.7 160.5,148.2 196.7,147.1 232.9,146.0 269.1,109.7 305.3,101.0 341.5,76.7 377.6,82.7 413.8,73.2 450.0,67.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,137.9 124.4,153.0 160.5,147.3 196.7,133.4 232.9,129.8 269.1,109.7 305.3,94.7 341.5,84.0 377.6,83.5 413.8,73.8 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.3 88.2,136.9 124.4,153.0 160.5,143.8 196.7,137.0 232.9,129.2 269.1,106.2 305.3,101.3 341.5,84.2 377.6,80.9 413.8,86.1 450.0,84.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,135.5 124.4,153.0 160.5,147.5 196.7,137.0 232.9,123.2 269.1,109.6 305.3,98.0 341.5,86.9 377.6,84.2 413.8,78.4 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.98 ns | 0.989 ns | 0.936 ns | 0.819 ns | 0.936 ns |
| D38 | 1.61 ns | 0.926 ns | 1.61 ns | 1.6 ns | 1.6 ns |
| D57 | 2.27 ns | 1.76 ns | 2.27 ns | 2.27 ns | 2.27 ns |
| D76 | 3.11 ns | 3.1 ns | 3.1 ns | 3.45 ns | 3.1 ns |
| D115 | 3.64 ns | 3.72 ns | 5.55 ns | 4.84 ns | 4.87 ns |
| D153 | 5.26 ns | 4.69 ns | 7.65 ns | 7.68 ns | 8.46 ns |
| D230 | 17.7 ns | 16.1 ns | 16.1 ns | 17.6 ns | 16.1 ns |
| D307 | 16.3 ns | 23.3 ns | 23.4 ns | 23.4 ns | 25.2 ns |
| D462 | 42.3 ns | 62.4 ns | 38.2 ns | 37.2 ns | 37.1 ns |
| D616 | 52.1 ns | 46.2 ns | 46.2 ns | 59.5 ns | 46 ns |
| D924 | 84.9 ns | 85.1 ns | 99.2 ns | 68.9 ns | 84.7 ns |
| D1232 | 121 ns | 106 ns | 106 ns | 90.2 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,160.4 88.2,149.7 124.4,142.2 160.5,135.4 196.7,131.9 232.9,123.9 269.1,97.7 305.3,99.4 341.5,78.7 377.6,74.1 413.8,63.6 450.0,55.9 450.0,58.6 413.8,63.6 377.6,76.9 341.5,81.5 305.3,89.9 269.1,99.6 232.9,113.6 196.7,125.6 160.5,135.4 124.4,142.2 88.2,149.7 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,160.4 88.2,149.7 124.4,142.2 160.5,135.4 196.7,131.9 232.9,123.9 269.1,97.7 305.3,99.4 341.5,78.7 377.6,74.1 413.8,63.6 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.2 88.2,161.7 124.4,147.8 160.5,135.4 196.7,131.5 232.9,126.5 269.1,99.6 305.3,91.6 341.5,70.2 377.6,76.7 413.8,63.5 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,142.2 160.5,135.4 196.7,122.8 232.9,115.8 269.1,99.6 305.3,91.6 341.5,80.9 377.6,76.8 413.8,60.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,149.8 124.4,142.2 160.5,133.1 196.7,125.7 232.9,115.7 269.1,97.7 305.3,91.6 341.5,81.5 377.6,71.3 413.8,68.1 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.7 124.4,142.2 160.5,135.4 196.7,125.6 232.9,113.6 269.1,99.6 305.3,89.9 341.5,81.5 377.6,76.9 413.8,63.6 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
