# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.937 ns | 0.937 ns | 1.05 ns | 0.935 ns | 1.05 ns |
| D38 | 1.62 ns | 1.62 ns | 1.6 ns | 1.62 ns | 0.989 ns |
| D57 | 2.25 ns | 2.5 ns | 2.5 ns | 2.24 ns | 2.49 ns |
| D76 | 3.09 ns | 3.09 ns | 3.09 ns | 3.48 ns | 1.63 ns |
| D115 | 4.42 ns | 4.42 ns | 3.54 ns | 4.39 ns | 2.35 ns |
| D153 | 5.94 ns | 5.96 ns | 4.54 ns | 4.35 ns | 3.85 ns |
| D230 | 12 ns | 13.8 ns | 15.4 ns | 11.9 ns | 13.7 ns |
| D307 | 19.6 ns | 11.8 ns | 19.6 ns | 19.6 ns | 18.6 ns |
| D462 | 33.2 ns | 33.4 ns | 55.7 ns | 29.2 ns | 34.3 ns |
| D616 | 45.1 ns | 40.1 ns | 45.9 ns | 40.7 ns | 79.1 ns |
| D924 | 56.9 ns | 83.1 ns | 84.8 ns | 74.5 ns | 84.9 ns |
| D1232 | 67.3 ns | 106 ns | 103 ns | 107 ns | 95.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.5 124.4,142.4 160.5,135.5 196.7,127.7 232.9,121.3 269.1,106.1 305.3,95.4 341.5,84.0 377.6,77.3 413.8,72.3 450.0,68.6 450.0,61.0 413.8,63.6 377.6,65.1 341.5,83.3 305.3,96.6 269.1,103.2 232.9,130.7 196.7,141.4 160.5,149.3 124.4,140.2 88.2,160.2 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.5 124.4,142.4 160.5,135.5 196.7,127.7 232.9,121.3 269.1,106.1 305.3,95.4 341.5,84.0 377.6,77.3 413.8,72.3 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,149.6 124.4,140.1 160.5,135.5 196.7,127.8 232.9,121.2 269.1,102.9 305.3,106.5 341.5,83.8 377.6,79.9 413.8,64.0 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.7 124.4,140.1 160.5,135.5 196.7,132.5 232.9,127.1 269.1,100.6 305.3,95.4 341.5,72.7 377.6,76.9 413.8,63.6 450.0,59.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.4 160.5,132.9 196.7,127.9 232.9,128.1 269.1,106.2 305.3,95.4 341.5,86.8 377.6,79.5 413.8,66.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,160.2 124.4,140.2 160.5,149.3 196.7,141.4 232.9,130.7 269.1,103.2 305.3,96.6 341.5,83.3 377.6,65.1 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.77 ns | 5.83 ns | 6.19 ns | 8.6 ns | 8.91 ns |
| D38 | 9.84 ns | 11.6 ns | 15.4 ns | 58.4 ns | 56.1 ns |
| D57 | 24 ns | 34.6 ns | 74.6 ns | 112 ns | 120 ns |
| D76 | 26.5 ns | 59.6 ns | 75.5 ns | 121 ns | 91.5 ns |
| D115 | 43.7 ns | 82.7 ns | 86.1 ns | 188 ns | 147 ns |
| D153 | 54.9 ns | 109 ns | 124 ns | 197 ns | 259 ns |
| D230 | 72.6 ns | 154 ns | 252 ns | 353 ns | 523 ns |
| D307 | 135 ns | 129 ns | 403 ns | 642 ns | 890 ns |
| D462 | 220 ns | 424 ns | 767 ns | 1.06 µs | 1.29 µs |
| D616 | 234 ns | 504 ns | 1.02 µs | 1.72 µs | 2.42 µs |
| D924 | 219 ns | 1.12 µs | 2.27 µs | 2.47 µs | 4.68 µs |
| D1232 | 311 ns | 1.94 µs | 3.47 µs | 4.72 µs | 7.23 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.9 88.2,160.3 124.4,141.0 160.5,138.8 196.7,128.0 232.9,123.0 269.1,116.9 305.3,103.5 341.5,92.9 377.6,91.5 413.8,92.9 450.0,85.4 450.0,17.0 413.8,26.5 377.6,40.8 341.5,54.5 305.3,62.5 269.1,74.1 232.9,89.3 196.7,101.7 160.5,111.9 124.4,106.1 88.2,122.6 52.0,162.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.9 88.2,160.3 124.4,141.0 160.5,138.8 196.7,128.0 232.9,123.0 269.1,116.9 305.3,103.5 341.5,92.9 377.6,91.5 413.8,92.9 450.0,85.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.7 88.2,156.8 124.4,133.1 160.5,121.2 196.7,114.1 232.9,108.2 269.1,100.6 305.3,104.4 341.5,78.6 377.6,74.9 413.8,57.5 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,150.7 124.4,116.4 160.5,116.1 196.7,113.2 232.9,105.4 269.1,89.9 305.3,79.7 341.5,65.8 377.6,59.6 413.8,42.2 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.3 88.2,121.7 124.4,107.6 160.5,105.9 196.7,96.2 232.9,95.3 269.1,82.6 305.3,69.6 341.5,58.8 377.6,48.2 413.8,40.3 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.5 88.2,122.6 124.4,106.1 160.5,111.9 196.7,101.7 232.9,89.3 269.1,74.1 305.3,62.5 341.5,54.5 377.6,40.8 413.8,26.5 450.0,17.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.87 ns | 3.2 ns | 4.93 ns | 4.95 ns |
| D38 | 3.5 ns | 13.9 ns | 26.1 ns | 27.1 ns | 18 ns |
| D57 | 6.54 ns | 21.7 ns | 34.4 ns | 73.2 ns | 76.5 ns |
| D76 | 7.79 ns | 34.5 ns | 41.9 ns | 83.6 ns | 63.5 ns |
| D115 | 13.1 ns | 52 ns | 69.8 ns | 195 ns | 149 ns |
| D153 | 18.4 ns | 52.7 ns | 92.8 ns | 202 ns | 283 ns |
| D230 | 21.7 ns | 116 ns | 368 ns | 441 ns | 788 ns |
| D307 | 54.3 ns | 94.5 ns | 509 ns | 1.08 µs | 1.39 µs |
| D462 | 99 ns | 426 ns | 1.35 µs | 1.75 µs | 2.09 µs |
| D616 | 92.7 ns | 506 ns | 1.75 µs | 2.65 µs | 4.21 µs |
| D924 | 112 ns | 1.5 µs | 3.2 µs | 5.11 µs | 8.3 µs |
| D1232 | 122 ns | 2.36 µs | 4.68 µs | 8.99 µs | 13.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,149.5 160.5,146.9 196.7,139.4 232.9,134.5 269.1,132.1 305.3,118.8 341.5,110.1 377.6,111.1 413.8,108.4 450.0,107.1 450.0,39.5 413.8,46.0 377.6,55.9 341.5,66.0 305.3,71.9 269.1,80.1 232.9,94.9 196.7,104.2 160.5,116.6 124.4,113.9 88.2,134.8 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,149.5 160.5,146.9 196.7,139.4 232.9,134.5 269.1,132.1 305.3,118.8 341.5,110.1 377.6,111.1 413.8,108.4 450.0,107.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,138.5 124.4,132.1 160.5,125.4 196.7,119.5 232.9,119.3 269.1,107.8 305.3,110.8 341.5,89.0 377.6,86.5 413.8,70.8 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.8 88.2,129.4 124.4,125.4 160.5,122.6 196.7,115.2 232.9,111.1 269.1,91.1 305.3,86.4 341.5,72.3 377.6,68.6 413.8,59.8 450.0,54.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,128.9 124.4,114.5 160.5,112.6 196.7,100.4 232.9,99.8 269.1,88.5 305.3,75.5 341.5,68.6 377.6,62.6 413.8,53.1 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,134.8 124.4,113.9 160.5,116.6 196.7,104.2 232.9,94.9 269.1,80.1 305.3,71.9 341.5,66.0 377.6,55.9 413.8,46.0 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.622 ns | 0.703 ns | 0.622 ns | 0.703 ns |
| D38 | 1.33 ns | 1.33 ns | 1.32 ns | 1.33 ns | 0.934 ns |
| D57 | 1.68 ns | 1.74 ns | 1.74 ns | 1.68 ns | 1.74 ns |
| D76 | 2.09 ns | 2.09 ns | 2.1 ns | 2.16 ns | 1.55 ns |
| D115 | 2.85 ns | 2.85 ns | 2.65 ns | 3.28 ns | 2.09 ns |
| D153 | 3.79 ns | 3.78 ns | 3.38 ns | 3.23 ns | 2.85 ns |
| D230 | 5.16 ns | 6 ns | 7.24 ns | 5.62 ns | 5.12 ns |
| D307 | 12.3 ns | 5.48 ns | 12.4 ns | 12.4 ns | 11 ns |
| D462 | 17.4 ns | 14.9 ns | 31.1 ns | 15 ns | 16.4 ns |
| D616 | 19 ns | 18.3 ns | 20 ns | 15 ns | 31.7 ns |
| D924 | 30.9 ns | 82.5 ns | 84.8 ns | 75.6 ns | 84.5 ns |
| D1232 | 32.2 ns | 69.7 ns | 68.8 ns | 69.7 ns | 61.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,121.9 196.7,113.0 232.9,104.8 269.1,95.8 305.3,70.7 341.5,60.6 377.6,58.1 413.8,44.0 450.0,42.8 450.0,24.1 413.8,14.9 377.6,43.3 341.5,62.3 305.3,73.9 269.1,96.0 232.9,113.0 196.7,122.0 160.5,130.7 124.4,127.3 88.2,145.3 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,121.9 196.7,113.0 232.9,104.8 269.1,95.8 305.3,70.7 341.5,60.6 377.6,58.1 413.8,44.0 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.1 124.4,127.3 160.5,121.9 196.7,113.0 232.9,104.8 269.1,91.5 305.3,94.1 341.5,65.1 377.6,59.2 413.8,15.6 450.0,20.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,121.9 196.7,115.1 232.9,108.1 269.1,86.0 305.3,70.6 341.5,43.8 377.6,56.7 413.8,14.8 450.0,20.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,128.3 160.5,121.0 196.7,108.9 232.9,109.4 269.1,93.4 305.3,70.6 341.5,65.0 377.6,64.8 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,145.3 124.4,127.3 160.5,130.7 196.7,122.0 232.9,113.0 269.1,96.0 305.3,73.9 341.5,62.3 377.6,43.3 413.8,14.9 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 2.11 ns | 2.18 ns | 2.46 ns |
| D38 | 3.25 ns | 3.24 ns | 3.26 ns | 3.17 ns | 2.15 ns |
| D57 | 7.17 ns | 8.09 ns | 8.09 ns | 7.17 ns | 8.08 ns |
| D76 | 8.43 ns | 8.71 ns | 8.71 ns | 9.83 ns | 4.32 ns |
| D115 | 12.8 ns | 12.8 ns | 9.37 ns | 12.8 ns | 6.87 ns |
| D153 | 16.4 ns | 16.1 ns | 12.8 ns | 11.2 ns | 10.9 ns |
| D230 | 28.4 ns | 32.2 ns | 36.2 ns | 28 ns | 22.8 ns |
| D307 | 48 ns | 23.5 ns | 47.9 ns | 47.8 ns | 42.6 ns |
| D462 | 90.4 ns | 73.5 ns | 110 ns | 74.1 ns | 62 ns |
| D616 | 94.1 ns | 63.6 ns | 80.3 ns | 69.4 ns | 113 ns |
| D924 | 65.2 ns | 110 ns | 99.6 ns | 88.4 ns | 83.6 ns |
| D1232 | 91.2 ns | 136 ns | 133 ns | 126 ns | 118 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,148.3 196.7,136.3 232.9,128.9 269.1,113.1 305.3,97.9 341.5,79.6 377.6,78.4 413.8,89.0 450.0,79.3 450.0,71.9 413.8,81.9 377.6,73.2 341.5,90.5 305.3,101.4 269.1,119.5 232.9,140.7 196.7,154.2 160.5,167.6 124.4,149.5 88.2,187.9 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,175.9 124.4,153.0 160.5,148.3 196.7,136.3 232.9,128.9 269.1,113.1 305.3,97.9 341.5,79.6 377.6,78.4 413.8,89.0 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,175.9 124.4,149.5 160.5,147.3 196.7,136.3 232.9,129.5 269.1,109.4 305.3,118.6 341.5,85.6 377.6,89.8 413.8,73.8 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,175.8 124.4,149.5 160.5,147.3 196.7,145.2 232.9,136.2 269.1,106.1 305.3,98.0 341.5,73.8 377.6,83.0 413.8,76.8 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.6 124.4,153.0 160.5,143.8 196.7,136.3 232.9,140.1 269.1,113.6 305.3,98.0 341.5,85.3 377.6,87.2 413.8,80.2 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,187.9 124.4,149.5 160.5,167.6 196.7,154.2 232.9,140.7 269.1,119.5 305.3,101.4 341.5,90.5 377.6,73.2 413.8,81.9 450.0,71.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 0.934 ns | 1.06 ns | 0.934 ns | 1.06 ns |
| D38 | 1.61 ns | 1.61 ns | 1.6 ns | 1.61 ns | 0.901 ns |
| D57 | 2.25 ns | 2.5 ns | 2.5 ns | 2.25 ns | 2.5 ns |
| D76 | 3.11 ns | 3.1 ns | 3.1 ns | 3.45 ns | 2 ns |
| D115 | 4.87 ns | 4.84 ns | 4.48 ns | 4.83 ns | 3.08 ns |
| D153 | 7.57 ns | 7.56 ns | 6.22 ns | 5.89 ns | 5.39 ns |
| D230 | 13.7 ns | 16.1 ns | 17.6 ns | 13.7 ns | 15 ns |
| D307 | 25.1 ns | 15.3 ns | 25.1 ns | 25.1 ns | 23.1 ns |
| D462 | 43.3 ns | 41.3 ns | 64.3 ns | 37.2 ns | 40.5 ns |
| D616 | 45.7 ns | 44.1 ns | 45.9 ns | 37.8 ns | 73.3 ns |
| D924 | 62.1 ns | 83.4 ns | 84.9 ns | 74.7 ns | 84.6 ns |
| D1232 | 73.8 ns | 106 ns | 104 ns | 107 ns | 95.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,149.6 124.4,142.3 160.5,135.3 196.7,125.6 232.9,116.0 269.1,103.1 305.3,90.0 341.5,78.2 377.6,77.0 413.8,70.3 450.0,66.6 450.0,60.9 413.8,63.6 377.6,66.7 341.5,79.6 305.3,91.9 269.1,101.2 232.9,123.4 196.7,135.6 160.5,145.0 124.4,140.1 88.2,162.3 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.3 160.5,135.3 196.7,125.6 232.9,116.0 269.1,103.1 305.3,90.0 341.5,78.2 377.6,77.0 413.8,70.3 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,135.5 196.7,125.7 232.9,116.1 269.1,99.6 305.3,100.8 341.5,79.2 377.6,77.8 413.8,63.9 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.8 124.4,140.1 160.5,135.4 196.7,127.5 232.9,120.3 269.1,97.7 305.3,90.0 341.5,69.6 377.6,76.9 413.8,63.5 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.4 160.5,133.1 196.7,125.8 232.9,121.5 269.1,103.2 305.3,90.0 341.5,81.5 377.6,81.1 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,162.3 124.4,140.1 160.5,145.0 196.7,135.6 232.9,123.4 269.1,101.2 305.3,91.9 341.5,79.6 377.6,66.7 413.8,63.6 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
