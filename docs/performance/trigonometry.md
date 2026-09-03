# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.04 ns | 2.02 µs | 3.3 µs | 2.31 µs | 3.63 µs |
| D38 | 1.31 µs | 4.68 µs | 4.59 µs | 8.02 µs | 9.74 µs |
| D57 | 1.44 µs | 3.6 µs | 6.78 µs | 10.5 µs | 14 µs |
| D76 | 1.44 µs | 6.57 µs | 9.86 µs | 15.4 µs | 18.5 µs |
| D115 | 1.47 µs | 4.91 µs | 21.2 µs | 26.9 µs | 40.1 µs |
| D153 | 982 ns | 10.3 µs | 25.4 µs | 45.8 µs | 63.2 µs |
| D230 | 1.36 µs | 16.7 µs | 44.9 µs | 69.4 µs | 123 µs |
| D307 | 1.49 µs | 18.6 µs | 66.1 µs | 72.7 µs | 107 µs |
| D462 | 1.64 µs | 42.7 µs | 127 µs | 256 µs | 375 µs |
| D616 | 1.62 µs | 66.7 µs | 207 µs | 427 µs | 564 µs |
| D924 | 1.48 µs | 127 µs | 309 µs | 591 µs | 1.67 ms |
| D1232 | 2.03 µs | 208 µs | 567 µs | 1.29 ms | 2.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.9 88.2,120.9 124.4,119.8 160.5,119.7 196.7,119.5 232.9,124.5 269.1,120.5 305.3,119.4 341.5,118.1 377.6,118.3 413.8,119.4 450.0,115.5 450.0,27.3 413.8,32.2 377.6,45.7 341.5,50.8 305.3,66.3 269.1,64.6 232.9,72.8 196.7,78.5 160.5,88.1 124.4,91.6 88.2,96.0 52.0,108.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.9 88.2,120.9 124.4,119.8 160.5,119.7 196.7,119.5 232.9,124.5 269.1,120.5 305.3,119.4 341.5,118.1 377.6,118.3 413.8,119.4 450.0,115.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.6 88.2,105.1 124.4,108.4 160.5,100.9 196.7,104.5 232.9,95.4 269.1,89.4 305.3,88.0 341.5,77.7 377.6,72.2 413.8,64.2 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,105.4 124.4,100.5 160.5,95.9 196.7,86.4 232.9,84.1 269.1,77.1 305.3,72.3 341.5,64.2 377.6,58.1 413.8,53.2 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.9 88.2,98.4 124.4,95.1 160.5,90.4 196.7,83.4 232.9,76.8 269.1,71.7 305.3,71.1 341.5,55.5 377.6,49.1 413.8,45.1 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,96.0 124.4,91.6 160.5,88.1 196.7,78.5 232.9,72.8 269.1,64.6 305.3,66.3 341.5,50.8 377.6,45.7 413.8,32.2 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.2 µs | 3.66 µs | 5.9 µs | 4.3 µs | 6.54 µs |
| D38 | 3.29 µs | 5.83 µs | 7.12 µs | 8.31 µs | 9.58 µs |
| D57 | 3.36 µs | 2.42 µs | 4.09 µs | 6.04 µs | 7.66 µs |
| D76 | 3.35 µs | 4.58 µs | 5.8 µs | 8.46 µs | 9.31 µs |
| D115 | 6.42 µs | 5.81 µs | 13.1 µs | 13.1 µs | 20.3 µs |
| D153 | 4.11 µs | 10.5 µs | 15.4 µs | 24.4 µs | 31.6 µs |
| D230 | 7.74 µs | 13.7 µs | 29.9 µs | 44.7 µs | 72 µs |
| D307 | 12.7 µs | 17.9 µs | 50.7 µs | 45.9 µs | 71.9 µs |
| D462 | 12.8 µs | 36.1 µs | 83.8 µs | 168 µs | 260 µs |
| D616 | 22.2 µs | 76.3 µs | 174 µs | 316 µs | 444 µs |
| D924 | 27.7 µs | 154 µs | 272 µs | 501 µs | 1.44 ms |
| D1232 | 44.9 µs | 268 µs | 579 µs | 1.22 ms | 2.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.9 88.2,184.1 124.4,183.7 160.5,183.8 196.7,169.6 232.9,179.3 269.1,165.6 305.3,154.9 341.5,154.6 377.6,142.7 413.8,137.9 450.0,127.4 450.0,42.9 413.8,52.1 377.6,77.7 341.5,89.3 305.3,117.2 269.1,117.1 232.9,135.0 196.7,144.6 160.5,161.5 124.4,165.8 88.2,160.9 52.0,169.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.9 88.2,184.1 124.4,183.7 160.5,183.8 196.7,169.6 232.9,179.3 269.1,165.6 305.3,154.9 341.5,154.6 377.6,142.7 413.8,137.9 450.0,127.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.8 88.2,171.7 124.4,190.8 160.5,177.0 196.7,171.8 232.9,159.0 269.1,153.2 305.3,147.4 341.5,132.1 377.6,115.9 413.8,100.7 450.0,88.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.5 88.2,167.4 124.4,179.4 160.5,171.8 196.7,154.1 232.9,150.6 269.1,136.2 305.3,124.7 341.5,113.8 377.6,98.0 413.8,88.3 450.0,71.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.3 88.2,164.0 124.4,170.9 160.5,163.6 196.7,154.2 232.9,140.6 269.1,127.5 305.3,126.9 341.5,98.8 377.6,85.0 413.8,75.0 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.2 88.2,160.9 124.4,165.8 160.5,161.5 196.7,144.6 232.9,135.0 269.1,117.1 305.3,117.2 341.5,89.3 377.6,77.7 413.8,52.1 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.05 ns | 2 µs | 3.27 µs | 2.3 µs | 3.65 µs |
| D38 | 1.18 µs | 4.67 µs | 4.53 µs | 8.06 µs | 9.65 µs |
| D57 | 1.31 µs | 3.59 µs | 6.73 µs | 10.4 µs | 14 µs |
| D76 | 1.34 µs | 6.58 µs | 9.77 µs | 15.3 µs | 18.5 µs |
| D115 | 1.37 µs | 4.88 µs | 19.9 µs | 26.9 µs | 39.9 µs |
| D153 | 892 ns | 10.2 µs | 25.7 µs | 45.2 µs | 61.9 µs |
| D230 | 1.25 µs | 16.5 µs | 44.9 µs | 69.6 µs | 123 µs |
| D307 | 1.38 µs | 18.6 µs | 65.3 µs | 73.7 µs | 105 µs |
| D462 | 1.55 µs | 42.7 µs | 126 µs | 255 µs | 373 µs |
| D616 | 1.45 µs | 66.6 µs | 207 µs | 428 µs | 564 µs |
| D924 | 1.4 µs | 127 µs | 308 µs | 590 µs | 1.67 ms |
| D1232 | 1.89 µs | 208 µs | 567 µs | 1.3 ms | 2.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.2 88.2,122.2 124.4,121.0 160.5,120.7 196.7,120.4 232.9,125.7 269.1,121.5 305.3,120.3 341.5,118.8 377.6,119.7 413.8,120.1 450.0,116.4 450.0,27.3 413.8,32.2 377.6,45.7 341.5,50.8 305.3,66.5 269.1,64.6 232.9,73.1 196.7,78.5 160.5,88.1 124.4,91.5 88.2,96.2 52.0,108.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.2 88.2,122.2 124.4,121.0 160.5,120.7 196.7,120.4 232.9,125.7 269.1,121.5 305.3,120.3 341.5,118.8 377.6,119.7 413.8,120.1 450.0,116.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.2 124.4,108.4 160.5,100.9 196.7,104.6 232.9,95.4 269.1,89.5 305.3,88.0 341.5,77.7 377.6,72.2 413.8,64.2 450.0,58.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,105.5 124.4,100.6 160.5,96.0 196.7,87.2 232.9,84.0 269.1,77.1 305.3,72.4 341.5,64.3 377.6,58.1 413.8,53.2 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.9 88.2,98.4 124.4,95.3 160.5,90.4 196.7,83.5 232.9,77.0 269.1,71.6 305.3,70.9 341.5,55.5 377.6,49.1 413.8,45.1 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.2 88.2,96.2 124.4,91.5 160.5,88.1 196.7,78.5 232.9,73.1 269.1,64.6 305.3,66.5 341.5,50.8 377.6,45.7 413.8,32.2 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2 ns | 1.73 µs | 2.61 µs | 1.87 µs | 2.87 µs |
| D38 | 3.74 ns | 2.53 µs | 3.16 µs | 3.72 µs | 4.24 µs |
| D57 | 1.95 ns | 3.03 µs | 6.09 µs | 9 µs | 11.8 µs |
| D76 | 2 ns | 6.43 µs | 8.93 µs | 12.9 µs | 14.4 µs |
| D115 | 14 ns | 8.57 µs | 18.8 µs | 19 µs | 31.2 µs |
| D153 | 8.1 ns | 15.2 µs | 22.1 µs | 36.1 µs | 44.5 µs |
| D230 | 26.2 ns | 20.1 µs | 44.8 µs | 62.1 µs | 95.5 µs |
| D307 | 45.3 ns | 26.2 µs | 72.5 µs | 65.4 µs | 95.8 µs |
| D462 | 75.2 ns | 55.4 µs | 119 µs | 209 µs | 320 µs |
| D616 | 79.6 ns | 113 µs | 225 µs | 403 µs | 534 µs |
| D924 | 98.5 ns | 229 µs | 354 µs | 571 µs | 1.6 ms |
| D1232 | 146 ns | 376 µs | 739 µs | 1.39 ms | 2.08 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.4 88.2,193.6 124.4,201.7 160.5,201.4 196.7,177.2 232.9,184.0 269.1,169.5 305.3,162.7 341.5,156.4 377.6,155.7 413.8,153.0 450.0,148.1 450.0,29.5 413.8,32.8 377.6,46.4 341.5,52.7 305.3,67.7 269.1,67.7 232.9,77.2 196.7,81.6 160.5,91.2 124.4,93.6 88.2,106.4 52.0,111.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.4 88.2,193.6 124.4,201.7 160.5,201.4 196.7,177.2 232.9,184.0 269.1,169.5 305.3,162.7 341.5,156.4 377.6,155.7 413.8,153.0 450.0,148.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,112.8 124.4,110.5 160.5,101.2 196.7,97.6 232.9,90.5 269.1,87.0 305.3,83.8 341.5,74.5 377.6,65.6 413.8,56.8 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,110.0 124.4,101.9 160.5,97.1 196.7,87.9 232.9,85.9 269.1,77.1 305.3,71.1 341.5,65.0 377.6,57.1 413.8,51.5 450.0,42.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.5 88.2,108.0 124.4,97.0 160.5,92.5 196.7,87.7 232.9,79.8 269.1,73.1 305.3,72.4 341.5,58.0 377.6,49.8 413.8,45.5 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,106.4 124.4,93.6 160.5,91.2 196.7,81.6 232.9,77.2 269.1,67.7 305.3,67.7 341.5,52.7 377.6,46.4 413.8,32.8 450.0,29.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.46 ns | 1.59 µs | 2.69 µs | 1.93 µs | 3.05 µs |
| D38 | 623 ns | 4.03 µs | 3.2 µs | 6.24 µs | 7.68 µs |
| D57 | 554 ns | 2.72 µs | 5.28 µs | 8.25 µs | 5.21 µs |
| D76 | 542 ns | 5.25 µs | 7.77 µs | 12.6 µs | 15.4 µs |
| D115 | 532 ns | 3.69 µs | 17 µs | 23.2 µs | 36.2 µs |
| D153 | 363 ns | 8.13 µs | 16.6 µs | 41.8 µs | 56.3 µs |
| D230 | 505 ns | 13.5 µs | 41 µs | 64.4 µs | 114 µs |
| D307 | 568 ns | 15.5 µs | 54.6 µs | 66.8 µs | 101 µs |
| D462 | 684 ns | 34.6 µs | 107 µs | 223 µs | 320 µs |
| D616 | 634 ns | 60 µs | 192 µs | 397 µs | 532 µs |
| D924 | 703 ns | 117 µs | 289 µs | 573 µs | 1.58 ms |
| D1232 | 991 ns | 192 µs | 533 µs | 1.25 ms | 2.37 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.6 88.2,130.2 124.4,131.6 160.5,131.9 196.7,132.1 232.9,136.8 269.1,132.8 305.3,131.3 341.5,129.0 377.6,129.9 413.8,128.7 450.0,124.4 450.0,27.9 413.8,32.9 377.6,46.4 341.5,52.7 305.3,67.0 269.1,65.5 232.9,74.3 196.7,79.7 160.5,90.3 124.4,103.8 88.2,99.0 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.6 88.2,130.2 124.4,131.6 160.5,131.9 196.7,132.1 232.9,136.8 269.1,132.8 305.3,131.3 341.5,129.0 377.6,129.9 413.8,128.7 450.0,124.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.6 88.2,107.0 124.4,111.9 160.5,103.7 196.7,108.1 232.9,98.3 269.1,92.0 305.3,90.3 341.5,80.3 377.6,73.5 413.8,65.2 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.8 124.4,103.6 160.5,98.9 196.7,89.1 232.9,89.4 269.1,78.2 305.3,74.6 341.5,66.3 377.6,59.1 413.8,54.0 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.1 88.2,101.6 124.4,98.1 160.5,92.9 196.7,85.3 232.9,78.0 269.1,72.6 305.3,72.2 341.5,57.2 377.6,50.0 413.8,45.5 450.0,35.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,99.0 124.4,103.8 160.5,90.3 196.7,79.7 232.9,74.3 269.1,65.5 305.3,67.0 341.5,52.7 377.6,46.4 413.8,32.9 450.0,27.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.31 ns | 1.77 µs | 2.9 µs | 2.15 µs | 3.3 µs |
| D38 | 3.73 ns | 2.89 µs | 3.6 µs | 4.15 µs | 4.84 µs |
| D57 | 445 ns | 3.15 µs | 5.72 µs | 8.64 µs | 11.5 µs |
| D76 | 450 ns | 6.13 µs | 8.18 µs | 12.1 µs | 14.1 µs |
| D115 | 935 ns | 8.19 µs | 19.4 µs | 19.6 µs | 31.8 µs |
| D153 | 601 ns | 15 µs | 23.8 µs | 38.3 µs | 51.5 µs |
| D230 | 1.07 µs | 19.7 µs | 46.9 µs | 75.5 µs | 124 µs |
| D307 | 1.89 µs | 28.2 µs | 84.2 µs | 77.4 µs | 124 µs |
| D462 | 2.09 µs | 56.2 µs | 144 µs | 298 µs | 470 µs |
| D616 | 3.51 µs | 123 µs | 305 µs | 565 µs | 809 µs |
| D924 | 4.28 µs | 258 µs | 482 µs | 921 µs | 2.67 ms |
| D1232 | 6.87 µs | 460 µs | 1.04 ms | 2.27 ms | 4.17 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.6 88.2,193.6 124.4,134.3 160.5,134.2 196.7,125.1 232.9,130.6 269.1,123.5 305.3,116.4 341.5,115.1 377.6,108.7 413.8,106.2 450.0,100.4 450.0,20.8 413.8,26.4 377.6,41.2 341.5,47.9 305.3,64.5 269.1,64.5 232.9,75.4 196.7,81.4 160.5,91.5 124.4,94.0 88.2,104.7 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.6 88.2,193.6 124.4,134.3 160.5,134.2 196.7,125.1 232.9,130.6 269.1,123.5 305.3,116.4 341.5,115.1 377.6,108.7 413.8,106.2 450.0,100.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,111.1 124.4,110.0 160.5,101.8 196.7,98.2 232.9,90.7 269.1,87.3 305.3,82.9 341.5,74.3 377.6,64.5 413.8,55.4 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,108.4 124.4,102.7 160.5,98.2 196.7,87.5 232.9,85.0 269.1,76.5 305.3,69.3 341.5,62.6 377.6,53.3 413.8,47.6 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.8 88.2,106.6 124.4,97.5 160.5,93.3 196.7,87.3 232.9,79.0 269.1,70.6 305.3,70.3 341.5,53.6 377.6,45.7 413.8,39.6 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,104.7 124.4,94.0 160.5,91.5 196.7,81.4 232.9,75.4 269.1,64.5 305.3,64.5 341.5,47.9 377.6,41.2 413.8,26.4 450.0,20.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.86 ns | 1.17 µs | 2.73 µs | 1.5 µs | 2.93 µs |
| D38 | 4.98 ns | 2.51 µs | 3.26 µs | 3.45 µs | 3.79 µs |
| D57 | 2.18 ns | 2.21 µs | 3.89 µs | 5.57 µs | 9.09 µs |
| D76 | 3.12 ns | 3.82 µs | 5.46 µs | 7.85 µs | 9.28 µs |
| D115 | 11.6 ns | 2.73 µs | 9.85 µs | 10.5 µs | 17 µs |
| D153 | 11.4 ns | 5.65 µs | 9.71 µs | 19.7 µs | 30.1 µs |
| D230 | 30.3 ns | 8.44 µs | 19.3 µs | 36.3 µs | 70.8 µs |
| D307 | 64.8 ns | 9.24 µs | 27.6 µs | 39.7 µs | 61.6 µs |
| D462 | 123 ns | 16.3 µs | 63.2 µs | 141 µs | 219 µs |
| D616 | 117 ns | 33.1 µs | 124 µs | 260 µs | 368 µs |
| D924 | 158 ns | 70.9 µs | 188 µs | 398 µs | 1.13 ms |
| D1232 | 362 ns | 123 µs | 366 µs | 900 µs | 1.79 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.0 88.2,190.1 124.4,200.3 160.5,195.9 196.7,179.6 232.9,179.8 269.1,167.7 305.3,158.2 341.5,150.3 377.6,150.9 413.8,147.2 450.0,136.9 450.0,31.4 413.8,37.1 377.6,51.0 341.5,57.4 305.3,73.2 269.1,71.4 232.9,82.0 196.7,89.1 160.5,96.6 124.4,96.9 88.2,107.8 52.0,110.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.0 88.2,190.1 124.4,200.3 160.5,195.9 196.7,179.6 232.9,179.8 269.1,167.7 305.3,158.2 341.5,150.3 377.6,150.9 413.8,147.2 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.3 88.2,112.9 124.4,114.4 160.5,107.7 196.7,111.8 232.9,102.8 269.1,97.8 305.3,96.7 341.5,89.7 377.6,80.9 413.8,71.4 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.8 88.2,109.6 124.4,107.4 160.5,103.2 196.7,95.9 232.9,96.1 269.1,87.5 305.3,83.1 341.5,72.8 377.6,64.5 413.8,59.3 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.3 88.2,108.9 124.4,103.0 160.5,98.7 196.7,95.1 232.9,87.3 269.1,79.7 305.3,78.6 341.5,62.9 377.6,55.3 413.8,50.0 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,107.8 124.4,96.9 160.5,96.6 196.7,89.1 232.9,82.0 269.1,71.4 305.3,73.2 341.5,57.4 377.6,51.0 413.8,37.1 450.0,31.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.24 ns | 1.78 µs | 3.7 µs | 2.18 µs | 3.96 µs |
| D38 | 4.05 ns | 3.36 µs | 4.36 µs | 4.46 µs | 4.94 µs |
| D57 | 2.8 ns | 3.5 µs | 5.9 µs | 8.07 µs | 10.4 µs |
| D76 | 3.43 ns | 5.95 µs | 7.83 µs | 11.3 µs | 12.4 µs |
| D115 | 10.3 ns | 7.82 µs | 12.1 µs | 16.7 µs | 24.5 µs |
| D153 | 11 ns | 7.99 µs | 15.6 µs | 24.9 µs | 36.4 µs |
| D230 | 30.3 ns | 11.6 µs | 24.5 µs | 43.3 µs | 79.4 µs |
| D307 | 65.1 ns | 12.4 µs | 55.2 µs | 43.7 µs | 64.6 µs |
| D462 | 123 ns | 23.4 µs | 81.3 µs | 167 µs | 232 µs |
| D616 | 117 ns | 39.8 µs | 134 µs | 271 µs | 353 µs |
| D924 | 173 ns | 82.3 µs | 200 µs | 401 µs | 994 µs |
| D1232 | 368 ns | 134 µs | 355 µs | 776 µs | 2.29 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.0 88.2,192.7 124.4,197.2 160.5,194.7 196.7,181.1 232.9,180.2 269.1,167.7 305.3,158.2 341.5,150.3 377.6,150.9 413.8,146.1 450.0,136.7 450.0,28.3 413.8,38.6 377.6,51.5 341.5,56.7 305.3,72.6 269.1,70.0 232.9,79.7 196.7,84.6 160.5,93.1 124.4,95.2 88.2,104.5 52.0,107.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.0 88.2,192.7 124.4,197.2 160.5,194.7 196.7,181.1 232.9,180.2 269.1,167.7 305.3,158.2 341.5,150.3 377.6,150.9 413.8,146.1 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.1 88.2,109.2 124.4,108.8 160.5,102.2 196.7,98.8 232.9,98.5 269.1,93.9 305.3,93.0 341.5,85.2 377.6,78.6 413.8,69.6 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,106.0 124.4,102.3 160.5,98.8 196.7,93.3 232.9,90.2 269.1,84.6 305.3,74.5 341.5,69.7 377.6,63.5 413.8,58.5 450.0,51.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.6 88.2,105.7 124.4,98.4 160.5,94.2 196.7,89.3 232.9,84.4 269.1,77.5 305.3,77.4 341.5,60.8 377.6,54.8 413.8,49.9 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,104.5 124.4,95.2 160.5,93.1 196.7,84.6 232.9,79.7 269.1,70.0 305.3,72.6 341.5,56.7 377.6,51.5 413.8,38.6 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.64 ns | 1.07 µs | 2.55 µs | 1.49 µs | 2.88 µs |
| D38 | 4.36 ns | 2.35 µs | 3.08 µs | 3.28 µs | 3.73 µs |
| D57 | 2.18 ns | 2.05 µs | 3.71 µs | 5.55 µs | 9.09 µs |
| D76 | 3.74 ns | 3.58 µs | 5.2 µs | 7.86 µs | 8.95 µs |
| D115 | 11.6 ns | 2.62 µs | 9.91 µs | 10.2 µs | 16.3 µs |
| D153 | 11.4 ns | 5.38 µs | 9.81 µs | 19 µs | 29.6 µs |
| D230 | 30.4 ns | 7.88 µs | 19.4 µs | 36.1 µs | 67 µs |
| D307 | 60.2 ns | 8.85 µs | 26.6 µs | 37.2 µs | 59.8 µs |
| D462 | 126 ns | 15.5 µs | 59.7 µs | 141 µs | 217 µs |
| D616 | 114 ns | 32.3 µs | 122 µs | 257 µs | 366 µs |
| D924 | 154 ns | 68.7 µs | 186 µs | 395 µs | 1.13 ms |
| D1232 | 345 ns | 122 µs | 364 µs | 888 µs | 1.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.9 88.2,191.7 124.4,200.3 160.5,193.6 196.7,179.6 232.9,179.8 269.1,167.6 305.3,159.2 341.5,150.0 377.6,151.3 413.8,147.5 450.0,137.5 450.0,31.5 413.8,37.1 377.6,51.0 341.5,57.6 305.3,73.5 269.1,72.1 232.9,82.3 196.7,89.7 160.5,97.1 124.4,96.9 88.2,107.9 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.9 88.2,191.7 124.4,200.3 160.5,193.6 196.7,179.6 232.9,179.8 269.1,167.6 305.3,159.2 341.5,150.0 377.6,151.3 413.8,147.5 450.0,137.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,113.7 124.4,115.4 160.5,108.5 196.7,112.4 232.9,103.4 269.1,98.7 305.3,97.2 341.5,90.3 377.6,81.2 413.8,71.8 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.3 124.4,108.0 160.5,103.8 196.7,95.8 232.9,96.0 269.1,87.5 305.3,83.6 341.5,73.6 377.6,64.7 413.8,59.5 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,109.6 124.4,103.0 160.5,98.7 196.7,95.5 232.9,87.7 269.1,79.8 305.3,79.4 341.5,62.9 377.6,55.4 413.8,50.1 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,107.9 124.4,96.9 160.5,97.1 196.7,89.7 232.9,82.3 269.1,72.1 305.3,73.5 341.5,57.6 377.6,51.0 413.8,37.1 450.0,31.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.42 ns | 1.79 µs | 3.7 µs | 2.18 µs | 4.06 µs |
| D38 | 4.05 ns | 3.37 µs | 4.36 µs | 4.47 µs | 4.94 µs |
| D57 | 10.6 ns | 3.51 µs | 5.94 µs | 8.1 µs | 10.5 µs |
| D76 | 10.4 ns | 5.95 µs | 7.86 µs | 11.3 µs | 12.4 µs |
| D115 | 10.6 ns | 7.77 µs | 11.9 µs | 17 µs | 25 µs |
| D153 | 10.7 ns | 8.07 µs | 15.4 µs | 24.7 µs | 36.4 µs |
| D230 | 29.3 ns | 11.6 µs | 24.6 µs | 43.5 µs | 79.6 µs |
| D307 | 60.5 ns | 12.4 µs | 53.6 µs | 43.9 µs | 64.6 µs |
| D462 | 125 ns | 23.6 µs | 81.3 µs | 168 µs | 232 µs |
| D616 | 111 ns | 39.8 µs | 134 µs | 271 µs | 353 µs |
| D924 | 170 ns | 82.1 µs | 200 µs | 408 µs | 993 µs |
| D1232 | 378 ns | 135 µs | 356 µs | 777 µs | 2.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.0 88.2,192.7 124.4,180.7 160.5,180.9 196.7,180.7 232.9,180.6 269.1,168.1 305.3,159.1 341.5,150.1 377.6,151.5 413.8,146.3 450.0,136.4 450.0,28.3 413.8,38.7 377.6,51.5 341.5,56.7 305.3,72.6 269.1,70.0 232.9,79.7 196.7,84.4 160.5,93.0 124.4,95.1 88.2,104.5 52.0,106.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.0 88.2,192.7 124.4,180.7 160.5,180.9 196.7,180.7 232.9,180.6 269.1,168.1 305.3,159.1 341.5,150.1 377.6,151.5 413.8,146.3 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.1 88.2,109.2 124.4,108.7 160.5,102.1 196.7,98.8 232.9,98.4 269.1,93.8 305.3,93.1 341.5,85.1 377.6,78.6 413.8,69.6 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,106.0 124.4,102.2 160.5,98.7 196.7,93.6 232.9,90.3 269.1,84.5 305.3,74.9 341.5,69.7 377.6,63.5 413.8,58.5 450.0,51.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.6 88.2,105.7 124.4,98.3 160.5,94.2 196.7,89.1 232.9,84.5 269.1,77.5 305.3,77.4 341.5,60.7 377.6,54.8 413.8,49.7 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,104.5 124.4,95.1 160.5,93.0 196.7,84.4 232.9,79.7 269.1,70.0 305.3,72.6 341.5,56.7 377.6,51.5 413.8,38.7 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.12 ns | 2.11 µs | 4.35 µs | 2.66 µs | 4.78 µs |
| D38 | 4.36 ns | 3.97 µs | 5.17 µs | 5.49 µs | 6.12 µs |
| D57 | 3.11 ns | 2.76 µs | 4.84 µs | 7.19 µs | 9.1 µs |
| D76 | 3.56 ns | 4.68 µs | 6.82 µs | 10 µs | 11.1 µs |
| D115 | 10.9 ns | 3.48 µs | 12.5 µs | 12.3 µs | 20 µs |
| D153 | 11 ns | 7.06 µs | 12.6 µs | 22.8 µs | 34.7 µs |
| D230 | 31.1 ns | 9.92 µs | 23.3 µs | 41.3 µs | 76.2 µs |
| D307 | 58.2 ns | 11 µs | 31.9 µs | 42.4 µs | 66.7 µs |
| D462 | 114 ns | 18.5 µs | 68.7 µs | 155 µs | 234 µs |
| D616 | 107 ns | 37.8 µs | 134 µs | 281 µs | 398 µs |
| D924 | 155 ns | 78.3 µs | 204 µs | 420 µs | 1.2 ms |
| D1232 | 375 ns | 134 µs | 394 µs | 948 µs | 1.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,191.7 124.4,195.9 160.5,194.2 196.7,180.4 232.9,180.2 269.1,167.3 305.3,159.6 341.5,151.2 377.6,152.0 413.8,147.4 450.0,136.5 450.0,30.9 413.8,36.4 377.6,50.0 341.5,56.6 305.3,72.2 269.1,70.5 232.9,80.3 196.7,87.1 160.5,94.4 124.4,96.9 88.2,101.8 52.0,104.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,191.7 124.4,195.9 160.5,194.2 196.7,180.4 232.9,180.2 269.1,167.3 305.3,159.6 341.5,151.2 377.6,152.0 413.8,147.4 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,107.2 124.4,111.7 160.5,105.1 196.7,108.8 232.9,100.0 269.1,95.8 305.3,94.5 341.5,88.1 377.6,79.2 413.8,70.2 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,103.9 124.4,104.7 160.5,100.5 196.7,92.9 232.9,92.9 269.1,85.2 305.3,81.3 341.5,71.8 377.6,63.5 413.8,58.3 450.0,50.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,103.2 124.4,99.8 160.5,95.7 196.7,93.1 232.9,85.5 269.1,78.1 305.3,77.8 341.5,61.7 377.6,54.3 413.8,49.3 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.9 88.2,101.8 124.4,96.9 160.5,94.4 196.7,87.1 232.9,80.3 269.1,70.5 305.3,72.2 341.5,56.6 377.6,50.0 413.8,36.4 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.82 ns | 1.81 µs | 3.88 µs | 2.31 µs | 4.17 µs |
| D38 | 3.74 ns | 3.55 µs | 4.39 µs | 4.5 µs | 4.97 µs |
| D57 | 2.6 µs | 3.6 µs | 6.18 µs | 8.46 µs | 10.9 µs |
| D76 | 2.57 µs | 6.15 µs | 8.24 µs | 11.7 µs | 12.8 µs |
| D115 | 5.34 µs | 8.08 µs | 12.5 µs | 17.2 µs | 25.3 µs |
| D153 | 1.73 µs | 8.45 µs | 16.3 µs | 25.4 µs | 37.4 µs |
| D230 | 2.51 µs | 12.1 µs | 25.5 µs | 44.4 µs | 80.7 µs |
| D307 | 2.95 µs | 12.8 µs | 55.1 µs | 44.8 µs | 65.1 µs |
| D462 | 3.36 µs | 24.3 µs | 82.7 µs | 170 µs | 235 µs |
| D616 | 3.21 µs | 41.5 µs | 135 µs | 275 µs | 357 µs |
| D924 | 3.17 µs | 83.8 µs | 204 µs | 401 µs | 1 ms |
| D1232 | 4.18 µs | 137 µs | 358 µs | 785 µs | 2.29 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.6 88.2,193.6 124.4,112.4 160.5,112.6 196.7,103.5 232.9,117.5 269.1,112.9 305.3,110.9 341.5,109.2 377.6,109.8 413.8,110.0 450.0,106.6 450.0,28.3 413.8,38.5 377.6,51.4 341.5,56.5 305.3,72.5 269.1,69.8 232.9,79.3 196.7,84.2 160.5,92.6 124.4,94.7 88.2,104.4 52.0,106.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.6 88.2,193.6 124.4,112.4 160.5,112.6 196.7,103.5 232.9,117.5 269.1,112.9 305.3,110.9 341.5,109.2 377.6,109.8 413.8,110.0 450.0,106.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,108.6 124.4,108.4 160.5,101.7 196.7,98.4 232.9,97.8 269.1,93.4 305.3,92.7 341.5,84.7 377.6,78.1 413.8,69.3 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,105.9 124.4,101.7 160.5,98.1 196.7,93.0 232.9,89.6 269.1,84.1 305.3,74.5 341.5,69.5 377.6,63.4 413.8,58.3 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.9 88.2,105.6 124.4,97.8 160.5,93.8 196.7,89.0 232.9,84.1 269.1,77.2 305.3,77.1 341.5,60.6 377.6,54.6 413.8,49.9 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.6 88.2,104.4 124.4,94.7 160.5,92.6 196.7,84.2 232.9,79.3 269.1,69.8 305.3,72.5 341.5,56.5 377.6,51.4 413.8,38.5 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.28 ns | 133 ns | 172 ns | 98.5 ns | 171 ns |
| D38 | 4.36 ns | 158 ns | 178 ns | 184 ns | 186 ns |
| D57 | 182 ns | 166 ns | 272 ns | 330 ns | 420 ns |
| D76 | 180 ns | 301 ns | 331 ns | 452 ns | 445 ns |
| D115 | 390 ns | 334 ns | 707 ns | 557 ns | 755 ns |
| D153 | 269 ns | 608 ns | 747 ns | 975 ns | 1.02 µs |
| D230 | 442 ns | 704 ns | 1.1 µs | 1.29 µs | 1.82 µs |
| D307 | 795 ns | 824 ns | 1.66 µs | 1.13 µs | 1.42 µs |
| D462 | 966 ns | 1.4 µs | 2.1 µs | 3.24 µs | 3.85 µs |
| D616 | 1.1 µs | 2.03 µs | 2.88 µs | 4.12 µs | 4.8 µs |
| D924 | 1.34 µs | 2.76 µs | 3.3 µs | 4.85 µs | 11.3 µs |
| D1232 | 2.24 µs | 4.08 µs | 5.85 µs | 9.29 µs | 25.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,184.4 124.4,119.6 160.5,119.8 196.7,106.4 232.9,112.8 269.1,104.2 305.3,94.0 341.5,90.6 377.6,88.4 413.8,84.9 450.0,76.0 450.0,33.8 413.8,47.9 377.6,62.8 341.5,66.6 305.3,83.9 269.1,79.6 232.9,89.7 196.7,94.9 160.5,104.1 124.4,105.1 88.2,119.2 52.0,120.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,184.4 124.4,119.6 160.5,119.8 196.7,106.4 232.9,112.8 269.1,104.2 305.3,94.0 341.5,90.6 377.6,88.4 413.8,84.9 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.1 88.2,122.0 124.4,121.1 160.5,110.9 196.7,109.1 232.9,98.6 269.1,96.1 305.3,93.4 341.5,84.1 377.6,77.7 413.8,72.3 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.5 88.2,120.0 124.4,112.6 160.5,109.2 196.7,96.0 232.9,95.1 269.1,88.4 305.3,81.2 341.5,77.1 377.6,71.6 413.8,69.3 450.0,59.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.3 88.2,119.4 124.4,109.3 160.5,103.8 196.7,100.2 232.9,90.4 269.1,85.5 305.3,87.8 341.5,69.6 377.6,65.4 413.8,62.6 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.7 88.2,119.2 124.4,105.1 160.5,104.1 196.7,94.9 232.9,89.7 269.1,79.6 305.3,83.9 341.5,66.6 377.6,62.8 413.8,47.9 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.31 ns | 155 ns | 211 ns | 126 ns | 201 ns |
| D38 | 4.36 ns | 194 ns | 214 ns | 196 ns | 198 ns |
| D57 | 284 ns | 241 ns | 352 ns | 457 ns | 539 ns |
| D76 | 274 ns | 421 ns | 450 ns | 569 ns | 572 ns |
| D115 | 646 ns | 438 ns | 932 ns | 742 ns | 934 ns |
| D153 | 391 ns | 865 ns | 989 ns | 1.2 µs | 1.3 µs |
| D230 | 747 ns | 954 ns | 1.46 µs | 1.73 µs | 2.23 µs |
| D307 | 1.34 µs | 1.13 µs | 2.27 µs | 1.45 µs | 1.77 µs |
| D462 | 1.52 µs | 1.84 µs | 2.73 µs | 3.83 µs | 4.51 µs |
| D616 | 1.81 µs | 2.65 µs | 3.67 µs | 4.89 µs | 5.48 µs |
| D924 | 2.05 µs | 3.63 µs | 3.95 µs | 5.55 µs | 12.3 µs |
| D1232 | 3.52 µs | 5.34 µs | 6.93 µs | 10.5 µs | 28.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.5 88.2,184.4 124.4,111.9 160.5,112.5 196.7,97.6 232.9,106.3 269.1,95.1 305.3,84.9 341.5,82.7 377.6,79.7 413.8,77.5 450.0,68.2 450.0,31.7 413.8,46.4 377.6,60.4 341.5,63.8 305.3,80.0 269.1,76.0 232.9,85.4 196.7,91.2 160.5,99.7 124.4,100.7 88.2,118.1 52.0,117.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.5 88.2,184.4 124.4,111.9 160.5,112.5 196.7,97.6 232.9,106.3 269.1,95.1 305.3,84.9 341.5,82.7 377.6,79.7 413.8,77.5 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.4 88.2,118.5 124.4,114.7 160.5,105.0 196.7,104.3 232.9,92.5 269.1,90.8 305.3,87.9 341.5,79.4 377.6,73.0 413.8,67.6 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.0 88.2,116.8 124.4,108.1 160.5,103.9 196.7,91.2 232.9,90.2 269.1,83.5 305.3,75.8 341.5,72.6 377.6,67.4 413.8,66.1 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,126.0 88.2,118.3 124.4,103.6 160.5,99.8 196.7,95.2 232.9,86.8 269.1,80.4 305.3,83.5 341.5,66.7 377.6,62.4 413.8,60.2 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.9 88.2,118.1 124.4,100.7 160.5,99.7 196.7,91.2 232.9,85.4 269.1,76.0 305.3,80.0 341.5,63.8 377.6,60.4 413.8,46.4 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
