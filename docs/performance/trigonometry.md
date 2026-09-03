# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8 ns | 1.35 µs | 3.07 µs | 3.76 µs | 3.76 µs |
| D38 | 930 ns | 5.16 µs | 5.17 µs | 5.66 µs | 10.3 µs |
| D57 | 1.42 µs | 6.36 µs | 8.79 µs | 11.2 µs | 15 µs |
| D76 | 1.57 µs | 4.16 µs | 9.87 µs | 15.3 µs | 12.9 µs |
| D115 | 1.23 µs | 8.27 µs | 17.9 µs | 33 µs | 44.8 µs |
| D153 | 1.6 µs | 10.2 µs | 24.7 µs | 42.4 µs | 65.4 µs |
| D230 | 1.6 µs | 21.1 µs | 42.4 µs | 70.4 µs | 124 µs |
| D307 | 1.02 µs | 21.9 µs | 60.8 µs | 133 µs | 182 µs |
| D462 | 1.52 µs | 42.3 µs | 125 µs | 223 µs | 399 µs |
| D616 | 1.34 µs | 61.8 µs | 222 µs | 452 µs | 711 µs |
| D924 | 1.66 µs | 127 µs | 450 µs | 745 µs | 1.55 ms |
| D1232 | 2.05 µs | 217 µs | 661 µs | 1.54 ms | 2.03 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,125.2 124.4,119.9 160.5,118.7 196.7,121.7 232.9,118.5 269.1,118.4 305.3,124.1 341.5,119.1 377.6,120.6 413.8,118.0 450.0,115.4 450.0,29.8 413.8,33.2 377.6,42.8 341.5,50.0 305.3,59.7 269.1,64.4 232.9,72.4 196.7,77.1 160.5,92.6 124.4,90.7 88.2,95.3 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,125.2 124.4,119.9 160.5,118.7 196.7,121.7 232.9,118.5 269.1,118.4 305.3,124.1 341.5,119.1 377.6,120.6 413.8,118.0 450.0,115.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.6 88.2,103.9 124.4,101.3 160.5,106.6 196.7,98.1 232.9,95.5 269.1,86.5 305.3,86.0 341.5,77.8 377.6,73.1 413.8,64.2 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,103.9 124.4,97.3 160.5,95.9 196.7,88.5 232.9,84.5 269.1,77.8 305.3,73.3 341.5,64.4 377.6,57.2 413.8,48.5 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,102.8 124.4,94.3 160.5,90.4 196.7,80.9 232.9,77.8 269.1,71.5 305.3,63.6 341.5,57.2 377.6,48.4 413.8,42.2 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,95.3 124.4,90.7 160.5,92.6 196.7,77.1 232.9,72.4 269.1,64.4 305.3,59.7 341.5,50.0 377.6,42.8 413.8,33.2 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.39 µs | 2.46 µs | 5.4 µs | 6.68 µs | 6.85 µs |
| D38 | 2.54 µs | 6.07 µs | 8.66 µs | 6.58 µs | 10.2 µs |
| D57 | 14.3 µs | 24.3 µs | 32.6 µs | 37 µs | 40.2 µs |
| D76 | 15.2 µs | 15.1 µs | 34 µs | 40 µs | 28.5 µs |
| D115 | 11.9 µs | 40.8 µs | 62.2 µs | 112 µs | 145 µs |
| D153 | 15.4 µs | 50.6 µs | 94.7 µs | 135 µs | 160 µs |
| D230 | 15.3 µs | 70.9 µs | 190 µs | 278 µs | 321 µs |
| D307 | 10.5 µs | 83.4 µs | 256 µs | 551 µs | 578 µs |
| D462 | 14.4 µs | 188 µs | 510 µs | 552 µs | 786 µs |
| D616 | 12.1 µs | 311 µs | 987 µs | 3.14 ms | 3.72 ms |
| D924 | 14.4 µs | 616 µs | 2.18 ms | 5.09 ms | 7.17 ms |
| D1232 | 16 µs | 985 µs | 7.16 ms | 8.71 ms | 7.1 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.5 88.2,189.7 124.4,152.3 160.5,150.9 196.7,156.2 232.9,150.6 269.1,150.8 305.3,158.9 341.5,152.0 377.6,155.9 413.8,152.1 450.0,149.7 450.0,17.4 413.8,17.2 377.6,31.5 341.5,65.2 305.3,71.9 269.1,84.7 232.9,99.8 196.7,101.9 160.5,137.3 124.4,129.8 88.2,159.6 52.0,168.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.5 88.2,189.7 124.4,152.3 160.5,150.9 196.7,156.2 232.9,150.6 269.1,150.8 305.3,158.9 341.5,152.0 377.6,155.9 413.8,152.1 450.0,149.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.4 88.2,170.8 124.4,140.7 160.5,151.1 196.7,129.5 232.9,124.8 269.1,117.5 305.3,113.9 341.5,96.3 377.6,85.4 413.8,70.5 450.0,60.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.4 88.2,163.1 124.4,134.3 160.5,133.4 196.7,120.3 232.9,111.2 269.1,96.0 305.3,89.6 341.5,74.6 377.6,60.3 413.8,43.1 450.0,17.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.7 88.2,169.1 124.4,131.6 160.5,129.9 196.7,107.6 232.9,103.5 269.1,87.8 305.3,72.9 341.5,72.9 377.6,35.2 413.8,24.7 450.0,13.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.2 88.2,159.6 124.4,129.8 160.5,137.3 196.7,101.9 232.9,99.8 269.1,84.7 305.3,71.9 341.5,65.2 377.6,31.5 413.8,17.2 450.0,17.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 1.33 µs | 3.09 µs | 3.73 µs | 3.75 µs |
| D38 | 839 ns | 5.13 µs | 5.07 µs | 5.61 µs | 10.3 µs |
| D57 | 1.33 µs | 6.36 µs | 8.7 µs | 11.2 µs | 14.9 µs |
| D76 | 1.47 µs | 4.16 µs | 9.85 µs | 15.3 µs | 12.9 µs |
| D115 | 1.14 µs | 8.19 µs | 18.1 µs | 32.7 µs | 46.2 µs |
| D153 | 1.49 µs | 10.2 µs | 24.7 µs | 42.2 µs | 64.8 µs |
| D230 | 1.53 µs | 21.2 µs | 42.4 µs | 70.4 µs | 124 µs |
| D307 | 940 ns | 21.9 µs | 61.3 µs | 133 µs | 181 µs |
| D462 | 1.44 µs | 42.8 µs | 126 µs | 222 µs | 399 µs |
| D616 | 1.26 µs | 61.9 µs | 225 µs | 452 µs | 717 µs |
| D924 | 1.54 µs | 127 µs | 449 µs | 745 µs | 1.55 ms |
| D1232 | 1.92 µs | 220 µs | 660 µs | 1.52 ms | 2.02 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,126.5 124.4,120.8 160.5,119.5 196.7,122.6 232.9,119.4 269.1,119.0 305.3,125.0 341.5,119.8 377.6,121.4 413.8,118.9 450.0,116.2 450.0,29.8 413.8,33.2 377.6,42.7 341.5,50.0 305.3,59.8 269.1,64.5 232.9,72.5 196.7,76.7 160.5,92.6 124.4,90.7 88.2,95.3 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,126.5 124.4,120.8 160.5,119.5 196.7,122.6 232.9,119.4 269.1,119.0 305.3,125.0 341.5,119.8 377.6,121.4 413.8,118.9 450.0,116.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.8 88.2,104.0 124.4,101.3 160.5,106.6 196.7,98.2 232.9,95.4 269.1,86.4 305.3,86.0 341.5,77.7 377.6,73.1 413.8,64.1 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,104.1 124.4,97.4 160.5,95.9 196.7,88.4 232.9,84.5 269.1,77.8 305.3,73.2 341.5,64.3 377.6,57.1 413.8,48.5 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,102.9 124.4,94.3 160.5,90.4 196.7,81.0 232.9,77.8 269.1,71.5 305.3,63.6 341.5,57.3 377.6,48.4 413.8,42.2 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,95.3 124.4,90.7 160.5,92.6 196.7,76.7 232.9,72.5 269.1,64.5 305.3,59.8 341.5,50.0 377.6,42.7 413.8,33.2 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.13 µs | 2.35 µs | 2.97 µs | 2.98 µs |
| D38 | 2.23 ns | 2.66 µs | 3.8 µs | 2.9 µs | 4.51 µs |
| D57 | 1.96 ns | 5.88 µs | 7.88 µs | 9.52 µs | 12.5 µs |
| D76 | 2.55 ns | 3.72 µs | 8.74 µs | 12.6 µs | 9.81 µs |
| D115 | 9.66 ns | 12.5 µs | 17.5 µs | 24.2 µs | 37 µs |
| D153 | 16 ns | 15.1 µs | 22.1 µs | 33.6 µs | 48.1 µs |
| D230 | 31.7 ns | 25.9 µs | 41.8 µs | 63.5 µs | 97 µs |
| D307 | 35.5 ns | 31.1 µs | 68 µs | 124 µs | 167 µs |
| D462 | 69.5 ns | 55.1 µs | 117 µs | 177 µs | 340 µs |
| D616 | 74.8 ns | 103 µs | 243 µs | 427 µs | 667 µs |
| D924 | 106 ns | 230 µs | 528 µs | 739 µs | 1.47 ms |
| D1232 | 167 ns | 386 µs | 859 µs | 1.63 ms | 1.79 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,200.1 124.4,201.7 160.5,198.4 196.7,181.9 232.9,175.6 269.1,167.1 305.3,165.7 341.5,157.4 377.6,156.5 413.8,152.2 450.0,146.5 450.0,31.4 413.8,33.8 377.6,43.6 341.5,52.0 305.3,60.8 269.1,67.5 232.9,76.2 196.7,79.5 160.5,95.9 124.4,92.9 88.2,105.6 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,200.1 124.4,201.7 160.5,198.4 196.7,181.9 232.9,175.6 269.1,167.1 305.3,165.7 341.5,157.4 377.6,156.5 413.8,152.2 450.0,146.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.8 88.2,112.1 124.4,102.3 160.5,108.0 196.7,93.0 232.9,90.6 269.1,83.9 305.3,81.6 341.5,74.5 377.6,66.7 413.8,56.8 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.7 88.2,107.7 124.4,98.7 160.5,97.4 196.7,88.8 232.9,85.9 269.1,78.0 305.3,71.9 341.5,65.1 377.6,56.1 413.8,46.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,111.1 124.4,96.3 160.5,92.8 196.7,84.7 232.9,80.7 269.1,72.8 305.3,64.5 341.5,60.1 377.6,49.1 413.8,42.3 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,105.6 124.4,92.9 160.5,95.9 196.7,79.5 232.9,76.2 269.1,67.5 305.3,60.8 341.5,52.0 377.6,43.6 413.8,33.8 450.0,31.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.07 µs | 2.58 µs | 3.13 µs | 3.22 µs |
| D38 | 427 ns | 4.47 µs | 3.69 µs | 4.39 µs | 8.27 µs |
| D57 | 517 ns | 4.84 µs | 6.79 µs | 8.9 µs | 5.53 µs |
| D76 | 592 ns | 3.33 µs | 7.68 µs | 12.6 µs | 10.7 µs |
| D115 | 470 ns | 6.34 µs | 14.8 µs | 29.7 µs | 40.7 µs |
| D153 | 617 ns | 7.87 µs | 16.5 µs | 38.1 µs | 59.9 µs |
| D230 | 647 ns | 18 µs | 38.4 µs | 66.1 µs | 115 µs |
| D307 | 385 ns | 19 µs | 49.4 µs | 123 µs | 169 µs |
| D462 | 643 ns | 34.4 µs | 106 µs | 194 µs | 340 µs |
| D616 | 619 ns | 56.4 µs | 207 µs | 423 µs | 673 µs |
| D924 | 765 ns | 116 µs | 420 µs | 717 µs | 1.46 ms |
| D1232 | 1.03 µs | 204 µs | 626 µs | 1.46 ms | 1.92 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,134.9 124.4,132.5 160.5,130.8 196.7,133.7 232.9,130.3 269.1,129.7 305.3,136.1 341.5,129.8 377.6,130.2 413.8,127.6 450.0,123.9 450.0,30.5 413.8,33.9 377.6,43.5 341.5,51.9 305.3,60.7 269.1,65.4 232.9,73.5 196.7,78.3 160.5,94.8 124.4,103.1 88.2,98.1 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,134.9 124.4,132.5 160.5,130.8 196.7,133.7 232.9,130.3 269.1,129.7 305.3,136.1 341.5,129.8 377.6,130.2 413.8,127.6 450.0,123.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,105.7 124.4,104.7 160.5,109.4 196.7,101.4 232.9,98.7 269.1,88.5 305.3,87.8 341.5,80.4 377.6,74.3 413.8,65.3 450.0,58.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,108.1 124.4,100.5 160.5,99.0 196.7,90.9 232.9,89.5 269.1,79.0 305.3,75.9 341.5,66.4 377.6,58.1 413.8,49.3 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.1 88.2,105.9 124.4,97.2 160.5,92.9 196.7,82.2 232.9,79.1 269.1,72.3 305.3,64.5 341.5,58.9 377.6,49.2 413.8,42.7 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,98.1 124.4,103.1 160.5,94.8 196.7,78.3 232.9,73.5 269.1,65.4 305.3,60.7 341.5,51.9 377.6,43.5 413.8,33.9 450.0,30.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 1.18 µs | 2.67 µs | 3.35 µs | 3.43 µs |
| D38 | 2.4 ns | 2.99 µs | 4.38 µs | 3.26 µs | 5.11 µs |
| D57 | 1.14 µs | 36.6 µs | 50.4 µs | 58 µs | 63.5 µs |
| D76 | 1.29 µs | 23 µs | 53.5 µs | 63.8 µs | 45.2 µs |
| D115 | 988 ns | 61.3 µs | 99.3 µs | 185 µs | 239 µs |
| D153 | 1.32 µs | 80.3 µs | 157 µs | 225 µs | 271 µs |
| D230 | 1.36 µs | 115 µs | 317 µs | 479 µs | 549 µs |
| D307 | 824 ns | 137 µs | 438 µs | 965 µs | 1.01 ms |
| D462 | 1.29 µs | 318 µs | 899 µs | 958 µs | 1.39 ms |
| D616 | 1.2 µs | 525 µs | 1.75 ms | 5.7 ms | 6.81 ms |
| D924 | 1.4 µs | 1.08 ms | 3.95 ms | 9.42 ms | 13.4 ms |
| D1232 | 2 µs | 1.75 ms | 13.3 ms | 16.2 ms | 13.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="185.0" x2="450" y2="185.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="188.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="135.0" x2="450" y2="135.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="138.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="85.0" x2="450" y2="85.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="88.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="35.0" x2="450" y2="35.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="38.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.4 88.2,200.5 124.4,133.6 160.5,132.2 196.7,135.1 232.9,132.0 269.1,131.6 305.3,137.1 341.5,132.3 377.6,133.0 413.8,131.4 450.0,127.5 450.0,32.0 413.8,31.9 377.6,39.2 341.5,56.4 305.3,59.9 269.1,66.5 232.9,74.2 196.7,75.6 160.5,93.6 124.4,89.9 88.2,117.3 52.0,121.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.4 88.2,200.5 124.4,133.6 160.5,132.2 196.7,135.1 232.9,132.0 269.1,131.6 305.3,137.1 341.5,132.3 377.6,133.0 413.8,131.4 450.0,127.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,133.2 88.2,123.1 124.4,95.9 160.5,101.0 196.7,90.3 232.9,87.4 269.1,83.5 305.3,81.6 341.5,72.4 377.6,67.0 413.8,59.2 450.0,53.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,119.0 124.4,92.4 160.5,91.8 196.7,85.1 232.9,80.1 269.1,72.5 305.3,69.0 341.5,61.2 377.6,53.9 413.8,45.1 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,122.2 124.4,90.9 160.5,89.9 196.7,78.3 232.9,76.2 269.1,68.0 305.3,60.4 341.5,60.5 377.6,41.1 413.8,35.6 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,117.3 124.4,89.9 160.5,93.6 196.7,75.6 232.9,74.2 269.1,66.5 305.3,59.9 341.5,56.4 377.6,39.2 413.8,31.9 450.0,32.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 886 ns | 2.4 µs | 2.83 µs | 2.86 µs |
| D38 | 2.66 ns | 2.7 µs | 3.77 µs | 2.36 µs | 4.13 µs |
| D57 | 2.18 ns | 3.76 µs | 5.16 µs | 6.21 µs | 9.6 µs |
| D76 | 3.17 ns | 2.37 µs | 5.49 µs | 7.85 µs | 6.19 µs |
| D115 | 10.1 ns | 4.68 µs | 9.06 µs | 14.1 µs | 19.1 µs |
| D153 | 18.2 ns | 5.38 µs | 10.5 µs | 18.4 µs | 32.1 µs |
| D230 | 40.8 ns | 10.3 µs | 18.8 µs | 36.7 µs | 70.5 µs |
| D307 | 49.3 ns | 10.9 µs | 24.6 µs | 74.4 µs | 110 µs |
| D462 | 124 ns | 15.6 µs | 60.8 µs | 125 µs | 232 µs |
| D616 | 142 ns | 30.3 µs | 130 µs | 274 µs | 457 µs |
| D924 | 150 ns | 72.2 µs | 272 µs | 489 µs | 1.05 ms |
| D1232 | 354 ns | 130 µs | 431 µs | 1.05 ms | 1.42 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,197.9 124.4,200.3 160.5,195.7 196.7,181.3 232.9,174.0 269.1,164.0 305.3,161.6 341.5,150.2 377.6,148.5 413.8,147.9 450.0,137.2 450.0,34.3 413.8,38.0 377.6,48.3 341.5,56.7 305.3,66.0 269.1,71.5 232.9,81.2 196.7,87.7 160.5,101.7 124.4,96.2 88.2,106.7 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,197.9 124.4,200.3 160.5,195.7 196.7,181.3 232.9,174.0 269.1,164.0 305.3,161.6 341.5,150.2 377.6,148.5 413.8,147.9 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.8 88.2,112.0 124.4,107.8 160.5,113.6 196.7,105.1 232.9,103.4 269.1,95.4 305.3,94.6 341.5,90.2 377.6,82.0 413.8,71.2 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.4 88.2,107.8 124.4,103.9 160.5,103.2 196.7,96.9 232.9,95.1 269.1,87.9 305.3,84.5 341.5,73.3 377.6,63.9 413.8,54.7 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,113.6 124.4,101.6 160.5,98.7 196.7,91.4 232.9,88.1 269.1,79.6 305.3,70.8 341.5,64.3 377.6,54.6 413.8,47.4 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,106.7 124.4,96.2 160.5,101.7 196.7,87.7 232.9,81.2 269.1,71.5 305.3,66.0 341.5,56.7 377.6,48.3 413.8,38.0 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.27 µs | 3.19 µs | 3.93 µs | 3.77 µs |
| D38 | 2.64 ns | 3.63 µs | 5.02 µs | 3.14 µs | 5.38 µs |
| D57 | 2.8 ns | 6.06 µs | 7.55 µs | 8.46 µs | 11 µs |
| D76 | 3.52 ns | 3.7 µs | 7.68 µs | 11.1 µs | 8.67 µs |
| D115 | 8.45 ns | 12.1 µs | 11.1 µs | 21.6 µs | 28.6 µs |
| D153 | 18 ns | 7.78 µs | 15.9 µs | 23.1 µs | 39.7 µs |
| D230 | 40.8 ns | 14.9 µs | 23.5 µs | 43.9 µs | 81.3 µs |
| D307 | 49.5 ns | 14.5 µs | 49.8 µs | 85.4 µs | 119 µs |
| D462 | 118 ns | 23.1 µs | 81.9 µs | 149 µs | 252 µs |
| D616 | 145 ns | 36.8 µs | 145 µs | 293 µs | 451 µs |
| D924 | 171 ns | 82.8 µs | 290 µs | 480 µs | 924 µs |
| D1232 | 374 ns | 143 µs | 416 µs | 926 µs | 1.72 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,198.0 124.4,197.2 160.5,194.4 196.7,183.5 232.9,174.2 269.1,164.0 305.3,161.6 341.5,150.8 377.6,148.2 413.8,146.2 450.0,136.5 450.0,31.8 413.8,39.6 377.6,48.5 341.5,55.7 305.3,65.0 269.1,69.7 232.9,78.6 196.7,82.7 160.5,97.5 124.4,94.5 88.2,103.4 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,198.0 124.4,197.2 160.5,194.4 196.7,183.5 232.9,174.2 269.1,164.0 305.3,161.6 341.5,150.8 377.6,148.2 413.8,146.2 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.4 88.2,108.3 124.4,101.9 160.5,108.1 196.7,93.3 232.9,98.8 269.1,90.7 305.3,91.1 341.5,85.3 377.6,79.5 413.8,69.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,104.3 124.4,99.2 160.5,99.0 196.7,94.4 232.9,90.0 269.1,85.1 305.3,75.8 341.5,69.6 377.6,62.5 413.8,53.9 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,110.1 124.4,97.8 160.5,94.4 196.7,86.1 232.9,85.3 269.1,77.4 305.3,69.1 341.5,62.2 377.6,53.8 413.8,47.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,103.4 124.4,94.5 160.5,97.5 196.7,82.7 232.9,78.6 269.1,69.7 305.3,65.0 341.5,55.7 377.6,48.5 413.8,39.6 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 795 ns | 2.24 µs | 2.77 µs | 2.79 µs |
| D38 | 2.24 ns | 2.53 µs | 3.57 µs | 2.24 µs | 4.09 µs |
| D57 | 2.49 ns | 3.55 µs | 4.88 µs | 5.95 µs | 9.61 µs |
| D76 | 3.52 ns | 2.32 µs | 5.19 µs | 7.82 µs | 5.99 µs |
| D115 | 10.1 ns | 4.37 µs | 8.6 µs | 14.3 µs | 18.9 µs |
| D153 | 18.1 ns | 5.1 µs | 9.97 µs | 18.8 µs | 32 µs |
| D230 | 40.8 ns | 10.1 µs | 19.2 µs | 36.5 µs | 68.1 µs |
| D307 | 43.8 ns | 10.8 µs | 23.9 µs | 70.9 µs | 108 µs |
| D462 | 121 ns | 15.3 µs | 58.3 µs | 124 µs | 228 µs |
| D616 | 126 ns | 29.6 µs | 127 µs | 271 µs | 458 µs |
| D924 | 160 ns | 68.9 µs | 269 µs | 485 µs | 1.05 ms |
| D1232 | 357 ns | 129 µs | 433 µs | 1.05 ms | 1.41 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,200.0 124.4,198.7 160.5,194.4 196.7,181.3 232.9,174.0 269.1,164.0 305.3,163.1 341.5,150.5 377.6,149.9 413.8,147.0 450.0,137.1 450.0,34.3 413.8,38.0 377.6,48.3 341.5,56.9 305.3,66.2 269.1,71.9 232.9,81.3 196.7,87.8 160.5,102.1 124.4,96.2 88.2,106.8 52.0,111.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,200.0 124.4,198.7 160.5,194.4 196.7,181.3 232.9,174.0 269.1,164.0 305.3,163.1 341.5,150.5 377.6,149.9 413.8,147.0 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,127.1 88.2,112.8 124.4,108.5 160.5,113.8 196.7,106.0 232.9,104.1 269.1,95.6 305.3,94.8 341.5,90.5 377.6,82.2 413.8,71.8 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.3 88.2,108.5 124.4,104.6 160.5,103.9 196.7,97.6 232.9,95.8 269.1,87.6 305.3,84.9 341.5,73.8 377.6,64.1 413.8,54.8 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,114.3 124.4,102.1 160.5,98.8 196.7,91.3 232.9,87.9 269.1,79.7 305.3,71.4 341.5,64.5 377.6,54.8 413.8,47.6 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,106.8 124.4,96.2 160.5,102.1 196.7,87.8 232.9,81.3 269.1,71.9 305.3,66.2 341.5,56.9 377.6,48.3 413.8,38.0 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.27 µs | 3.19 µs | 3.92 µs | 3.77 µs |
| D38 | 2.44 ns | 3.63 µs | 5.02 µs | 3.15 µs | 5.39 µs |
| D57 | 10.6 ns | 6.01 µs | 7.57 µs | 8.47 µs | 11.1 µs |
| D76 | 12.1 ns | 3.67 µs | 7.71 µs | 11.1 µs | 8.69 µs |
| D115 | 8.46 ns | 12.3 µs | 10.8 µs | 21.8 µs | 28.3 µs |
| D153 | 17.4 ns | 7.88 µs | 15.7 µs | 23.2 µs | 39.1 µs |
| D230 | 39.9 ns | 15 µs | 23.6 µs | 44 µs | 81.4 µs |
| D307 | 43.7 ns | 14.5 µs | 49.6 µs | 86.2 µs | 119 µs |
| D462 | 112 ns | 23.2 µs | 81.5 µs | 149 µs | 250 µs |
| D616 | 121 ns | 36.9 µs | 144 µs | 291 µs | 450 µs |
| D924 | 172 ns | 82.9 µs | 292 µs | 480 µs | 925 µs |
| D1232 | 360 ns | 144 µs | 416 µs | 926 µs | 1.73 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,198.9 124.4,180.7 160.5,179.1 196.7,183.5 232.9,174.6 269.1,164.3 305.3,163.1 341.5,151.5 377.6,150.5 413.8,146.1 450.0,137.0 450.0,31.8 413.8,39.5 377.6,48.5 341.5,55.8 305.3,65.0 269.1,69.7 232.9,78.8 196.7,82.8 160.5,97.5 124.4,94.4 88.2,103.4 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,198.9 124.4,180.7 160.5,179.1 196.7,183.5 232.9,174.6 269.1,164.3 305.3,163.1 341.5,151.5 377.6,150.5 413.8,146.1 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.3 88.2,108.3 124.4,102.0 160.5,108.1 196.7,93.1 232.9,98.7 269.1,90.7 305.3,91.1 341.5,85.3 377.6,79.5 413.8,69.5 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,104.3 124.4,99.2 160.5,98.9 196.7,94.8 232.9,90.1 269.1,85.0 305.3,75.9 341.5,69.7 377.6,62.6 413.8,53.9 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,110.1 124.4,97.8 160.5,94.4 196.7,86.1 232.9,85.3 269.1,77.3 305.3,69.0 341.5,62.2 377.6,53.9 413.8,47.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,103.4 124.4,94.4 160.5,97.5 196.7,82.8 232.9,78.8 269.1,69.7 305.3,65.0 341.5,55.8 377.6,48.5 413.8,39.5 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 1.61 µs | 3.84 µs | 4.54 µs | 4.61 µs |
| D38 | 2.24 ns | 4.29 µs | 5.97 µs | 4.1 µs | 6.65 µs |
| D57 | 2.81 ns | 4.78 µs | 6.49 µs | 7.81 µs | 9.9 µs |
| D76 | 4.22 ns | 2.95 µs | 7.05 µs | 9.98 µs | 7.49 µs |
| D115 | 8.09 ns | 5.8 µs | 10.9 µs | 17.8 µs | 23 µs |
| D153 | 18 ns | 6.91 µs | 12.3 µs | 22.2 µs | 37.4 µs |
| D230 | 41.3 ns | 12.8 µs | 23.8 µs | 42.5 µs | 77.2 µs |
| D307 | 43.3 ns | 13.4 µs | 28.3 µs | 80.4 µs | 120 µs |
| D462 | 96.4 ns | 18.7 µs | 67 µs | 137 µs | 247 µs |
| D616 | 126 ns | 34.6 µs | 145 µs | 296 µs | 494 µs |
| D924 | 145 ns | 77.7 µs | 294 µs | 522 µs | 1.12 ms |
| D1232 | 384 ns | 141 µs | 464 µs | 1.11 ms | 1.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,200.0 124.4,197.2 160.5,192.1 196.7,184.1 232.9,174.1 269.1,163.8 305.3,163.3 341.5,153.3 377.6,150.0 413.8,148.3 450.0,136.2 450.0,33.7 413.8,37.2 377.6,47.3 341.5,55.9 305.3,64.9 269.1,70.4 232.9,79.4 196.7,85.4 160.5,99.3 124.4,95.8 88.2,100.8 52.0,105.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,200.0 124.4,197.2 160.5,192.1 196.7,184.1 232.9,174.1 269.1,163.8 305.3,163.3 341.5,153.3 377.6,150.0 413.8,148.3 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.4 88.2,106.2 124.4,104.9 160.5,110.9 196.7,102.5 232.9,100.3 269.1,92.6 305.3,92.1 341.5,87.9 377.6,80.3 413.8,70.3 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,102.1 124.4,101.1 160.5,100.1 196.7,94.7 232.9,93.1 269.1,85.0 305.3,82.8 341.5,72.1 377.6,62.6 413.8,53.8 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,106.8 124.4,98.8 160.5,95.7 196.7,88.6 232.9,85.8 269.1,77.8 305.3,69.9 341.5,63.2 377.6,53.7 413.8,46.6 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,100.8 124.4,95.8 160.5,99.3 196.7,85.4 232.9,79.4 269.1,70.4 305.3,64.9 341.5,55.9 377.6,47.3 413.8,37.2 450.0,33.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 1.29 µs | 3.37 µs | 4.09 µs | 3.94 µs |
| D38 | 2.46 ns | 3.82 µs | 5.1 µs | 3.15 µs | 5.4 µs |
| D57 | 2.6 µs | 6.24 µs | 7.91 µs | 8.83 µs | 11.5 µs |
| D76 | 2.77 µs | 3.89 µs | 8.15 µs | 11.5 µs | 8.96 µs |
| D115 | 4.92 µs | 12.8 µs | 11.3 µs | 22.5 µs | 29.5 µs |
| D153 | 2.96 µs | 8.26 µs | 16.7 µs | 23.8 µs | 40.1 µs |
| D230 | 3.02 µs | 15.4 µs | 24.1 µs | 45.4 µs | 82.4 µs |
| D307 | 2.12 µs | 15 µs | 51.7 µs | 87.2 µs | 120 µs |
| D462 | 3.14 µs | 23.9 µs | 83.1 µs | 150 µs | 255 µs |
| D616 | 2.83 µs | 37.8 µs | 146 µs | 294 µs | 453 µs |
| D924 | 3.46 µs | 84 µs | 295 µs | 483 µs | 931 µs |
| D1232 | 4.43 µs | 145 µs | 421 µs | 933 µs | 1.73 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,198.8 124.4,112.5 160.5,111.7 196.7,104.5 232.9,110.8 269.1,110.6 305.3,115.0 341.5,110.1 377.6,111.4 413.8,108.9 450.0,105.8 450.0,31.7 413.8,39.5 377.6,48.4 341.5,55.6 305.3,64.9 269.1,69.6 232.9,78.5 196.7,82.3 160.5,97.1 124.4,93.9 88.2,103.3 52.0,107.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,198.8 124.4,112.5 160.5,111.7 196.7,104.5 232.9,110.8 269.1,110.6 305.3,115.0 341.5,110.1 377.6,111.4 413.8,108.9 450.0,105.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.1 88.2,107.7 124.4,101.6 160.5,107.4 196.7,92.6 232.9,98.1 269.1,90.4 305.3,90.7 341.5,84.9 377.6,79.2 413.8,69.3 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,104.1 124.4,98.6 160.5,98.3 196.7,94.2 232.9,89.4 269.1,84.8 305.3,75.3 341.5,69.4 377.6,62.4 413.8,53.7 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,110.0 124.4,97.3 160.5,94.0 196.7,85.6 232.9,84.9 269.1,76.9 305.3,68.8 341.5,62.1 377.6,53.8 413.8,47.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,103.3 124.4,93.9 160.5,97.1 196.7,82.3 232.9,78.5 269.1,69.6 305.3,64.9 341.5,55.6 377.6,48.4 413.8,39.5 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 78.7 ns | 147 ns | 173 ns | 162 ns |
| D38 | 2.35 ns | 170 ns | 207 ns | 132 ns | 204 ns |
| D57 | 178 ns | 336 ns | 346 ns | 347 ns | 444 ns |
| D76 | 207 ns | 170 ns | 325 ns | 443 ns | 309 ns |
| D115 | 391 ns | 567 ns | 606 ns | 790 ns | 997 ns |
| D153 | 443 ns | 567 ns | 684 ns | 838 ns | 1.13 µs |
| D230 | 590 ns | 900 ns | 991 ns | 1.27 µs | 1.83 µs |
| D307 | 552 ns | 1.03 µs | 1.5 µs | 2.25 µs | 2.67 µs |
| D462 | 882 ns | 1.4 µs | 2.18 µs | 2.93 µs | 4.18 µs |
| D616 | 1.01 µs | 1.89 µs | 3.11 µs | 4.46 µs | 6.07 µs |
| D924 | 1.45 µs | 2.78 µs | 4.84 µs | 6.04 µs | 10.3 µs |
| D1232 | 2.3 µs | 4.21 µs | 6.93 µs | 11.1 µs | 18.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,195.1 124.4,120.0 160.5,117.4 196.7,106.3 232.9,104.1 269.1,99.2 305.3,100.3 341.5,92.2 377.6,89.8 413.8,83.5 450.0,75.5 450.0,39.0 413.8,49.4 377.6,58.7 341.5,65.1 305.3,72.9 269.1,79.5 232.9,87.9 196.7,90.1 160.5,110.4 124.4,104.1 88.2,117.6 52.0,121.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,195.1 124.4,120.0 160.5,117.4 196.7,106.3 232.9,104.1 269.1,99.2 305.3,100.3 341.5,92.2 377.6,89.8 413.8,83.5 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,134.2 88.2,120.8 124.4,108.9 160.5,120.8 196.7,99.8 232.9,99.9 269.1,91.8 305.3,89.4 341.5,84.2 377.6,78.9 413.8,72.2 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.4 88.2,117.4 124.4,108.4 160.5,109.5 196.7,98.7 232.9,96.6 269.1,90.2 305.3,82.9 341.5,76.5 377.6,70.3 413.8,62.6 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.4 88.2,125.2 124.4,108.4 160.5,104.2 196.7,94.1 232.9,93.1 269.1,85.9 305.3,75.9 341.5,71.3 377.6,64.0 413.8,58.7 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.7 88.2,117.6 124.4,104.1 160.5,110.4 196.7,90.1 232.9,87.9 269.1,79.5 305.3,72.9 341.5,65.1 377.6,58.7 413.8,49.4 450.0,39.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 106 ns | 190 ns | 215 ns | 199 ns |
| D38 | 2.62 ns | 209 ns | 251 ns | 159 ns | 220 ns |
| D57 | 275 ns | 437 ns | 448 ns | 463 ns | 565 ns |
| D76 | 318 ns | 232 ns | 458 ns | 562 ns | 391 ns |
| D115 | 576 ns | 776 ns | 782 ns | 1.03 µs | 1.2 µs |
| D153 | 692 ns | 816 ns | 919 ns | 1.03 µs | 1.37 µs |
| D230 | 942 ns | 1.26 µs | 1.31 µs | 1.63 µs | 2.19 µs |
| D307 | 902 ns | 1.45 µs | 2.05 µs | 2.86 µs | 3.26 µs |
| D462 | 1.44 µs | 1.88 µs | 2.79 µs | 3.5 µs | 4.91 µs |
| D616 | 1.57 µs | 2.44 µs | 3.86 µs | 5.28 µs | 6.96 µs |
| D924 | 2.3 µs | 3.63 µs | 5.82 µs | 6.8 µs | 11.4 µs |
| D1232 | 3.57 µs | 5.44 µs | 8.11 µs | 12.5 µs | 19.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,193.3 124.4,112.4 160.5,109.9 196.7,99.6 232.9,96.4 269.1,91.0 305.3,91.8 341.5,83.7 377.6,82.2 413.8,75.5 450.0,67.9 450.0,38.2 413.8,47.7 377.6,56.3 341.5,62.4 305.3,69.5 269.1,76.4 232.9,84.5 196.7,86.8 160.5,106.3 124.4,99.9 88.2,116.3 52.0,118.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,193.3 124.4,112.4 160.5,109.9 196.7,99.6 232.9,96.4 269.1,91.0 305.3,91.8 341.5,83.7 377.6,82.2 413.8,75.5 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,129.0 88.2,117.2 124.4,104.4 160.5,115.4 196.7,94.4 232.9,93.5 269.1,85.9 305.3,83.5 341.5,79.1 377.6,74.5 413.8,67.6 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.8 88.2,114.0 124.4,104.0 160.5,103.6 196.7,94.3 232.9,91.5 269.1,85.4 305.3,77.5 341.5,72.2 377.6,66.5 413.8,59.4 450.0,53.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.7 88.2,122.0 124.4,103.4 160.5,100.0 196.7,89.6 232.9,89.5 269.1,81.5 305.3,71.7 341.5,68.3 377.6,61.1 413.8,56.7 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.1 88.2,116.3 124.4,99.9 160.5,106.3 196.7,86.8 232.9,84.5 269.1,76.4 305.3,69.5 341.5,62.4 377.6,56.3 413.8,47.7 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
