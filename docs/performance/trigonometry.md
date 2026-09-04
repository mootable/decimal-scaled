# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.8 ns | 1.73 µs | 3.24 µs | 3.49 µs | 3.29 µs |
| D38 | 1.32 µs | 4 µs | 4.52 µs | 8.08 µs | 9.67 µs |
| D57 | 1.54 µs | 4.31 µs | 8.2 µs | 8.59 µs | 14.3 µs |
| D76 | 1 µs | 5.83 µs | 10.3 µs | 15 µs | 19.1 µs |
| D115 | 1.46 µs | 8.74 µs | 20 µs | 32.3 µs | 42 µs |
| D153 | 1.12 µs | 10.6 µs | 15.8 µs | 32.8 µs | 60 µs |
| D230 | 1.66 µs | 18.3 µs | 42.5 µs | 69.8 µs | 123 µs |
| D307 | 1.45 µs | 24.6 µs | 66.5 µs | 124 µs | 182 µs |
| D462 | 1.22 µs | 29.8 µs | 125 µs | 240 µs | 374 µs |
| D616 | 1.25 µs | 60.8 µs | 207 µs | 447 µs | 613 µs |
| D924 | 1.8 µs | 126 µs | 277 µs | 876 µs | 1.67 ms |
| D1232 | 1.28 µs | 175 µs | 658 µs | 1.53 ms | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,120.8 124.4,118.9 160.5,124.2 196.7,119.6 232.9,122.9 269.1,118.0 305.3,119.7 341.5,121.8 377.6,121.5 413.8,117.0 450.0,121.3 450.0,26.0 413.8,32.2 377.6,44.6 341.5,50.8 305.3,59.7 269.1,64.5 232.9,73.5 196.7,77.9 160.5,87.7 124.4,91.3 88.2,96.1 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,120.8 124.4,118.9 160.5,124.2 196.7,119.6 232.9,122.9 269.1,118.0 305.3,119.7 341.5,121.8 377.6,121.5 413.8,117.0 450.0,121.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,107.1 124.4,106.2 160.5,102.4 196.7,97.4 232.9,94.9 269.1,88.2 305.3,84.5 341.5,82.1 377.6,73.3 413.8,64.3 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,105.6 124.4,98.2 160.5,95.3 196.7,87.1 232.9,90.1 269.1,77.7 305.3,72.2 341.5,64.4 377.6,58.1 413.8,54.5 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,97.6 160.5,90.7 196.7,81.2 232.9,81.0 269.1,71.6 305.3,64.4 341.5,56.3 377.6,48.6 413.8,40.2 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,96.1 124.4,91.3 160.5,87.7 196.7,77.9 232.9,73.5 269.1,64.5 305.3,59.7 341.5,50.8 377.6,44.6 413.8,32.2 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.22 µs | 3.06 µs | 5.85 µs | 6.4 µs | 5.77 µs |
| D38 | 3.22 µs | 4.86 µs | 7.4 µs | 8.43 µs | 9.75 µs |
| D57 | 3.75 µs | 3 µs | 4.98 µs | 4.97 µs | 7.72 µs |
| D76 | 2.33 µs | 3.93 µs | 5.92 µs | 7.95 µs | 9.46 µs |
| D115 | 6.11 µs | 9.5 µs | 13.7 µs | 16.5 µs | 21.8 µs |
| D153 | 4.82 µs | 10.6 µs | 9.53 µs | 16.4 µs | 30.7 µs |
| D230 | 8.88 µs | 14.1 µs | 26.9 µs | 42.9 µs | 69.2 µs |
| D307 | 12.6 µs | 26.7 µs | 51.4 µs | 85.8 µs | 135 µs |
| D462 | 10 µs | 23.6 µs | 84.4 µs | 161 µs | 261 µs |
| D616 | 17.4 µs | 72.2 µs | 170 µs | 331 µs | 491 µs |
| D924 | 33.9 µs | 153 µs | 249 µs | 757 µs | 1.43 ms |
| D1232 | 23 µs | 217 µs | 684 µs | 1.46 ms | 2.41 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,184.6 124.4,181.3 160.5,191.7 196.7,170.7 232.9,175.8 269.1,162.6 305.3,154.9 341.5,159.9 377.6,147.9 413.8,133.5 450.0,141.9 450.0,40.9 413.8,52.2 377.6,75.4 341.5,89.1 305.3,103.5 269.1,118.0 232.9,135.7 196.7,143.1 160.5,161.2 124.4,165.6 88.2,160.6 52.0,172.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,184.6 124.4,181.3 160.5,191.7 196.7,170.7 232.9,175.8 269.1,162.6 305.3,154.9 341.5,159.9 377.6,147.9 413.8,133.5 450.0,141.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,185.7 88.2,175.6 124.4,186.1 160.5,180.3 196.7,161.1 232.9,158.7 269.1,152.6 305.3,138.7 341.5,141.4 377.6,117.1 413.8,100.7 450.0,93.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,166.5 124.4,175.1 160.5,171.4 196.7,153.2 232.9,161.1 269.1,138.5 305.3,124.5 341.5,113.7 377.6,98.5 413.8,90.2 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,163.7 124.4,175.2 160.5,165.0 196.7,149.1 232.9,149.2 269.1,128.4 305.3,113.3 341.5,99.7 377.6,84.0 413.8,66.0 450.0,51.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.0 88.2,160.6 124.4,165.6 160.5,161.2 196.7,143.1 232.9,135.7 269.1,118.0 305.3,103.5 341.5,89.1 377.6,75.4 413.8,52.2 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.71 µs | 3.23 µs | 3.5 µs | 3.29 µs |
| D38 | 1.18 µs | 3.98 µs | 4.45 µs | 8.07 µs | 9.62 µs |
| D57 | 1.44 µs | 4.3 µs | 8.18 µs | 8.56 µs | 14.2 µs |
| D76 | 944 ns | 5.8 µs | 10.2 µs | 14.9 µs | 19.1 µs |
| D115 | 1.37 µs | 8.71 µs | 20.9 µs | 32.6 µs | 42 µs |
| D153 | 1.02 µs | 10.6 µs | 16 µs | 32.8 µs | 59.8 µs |
| D230 | 1.55 µs | 18.9 µs | 42.7 µs | 69.7 µs | 124 µs |
| D307 | 1.34 µs | 25.4 µs | 66.9 µs | 125 µs | 181 µs |
| D462 | 1.15 µs | 29 µs | 125 µs | 239 µs | 374 µs |
| D616 | 1.17 µs | 60.5 µs | 206 µs | 448 µs | 616 µs |
| D924 | 1.69 µs | 125 µs | 276 µs | 880 µs | 1.67 ms |
| D1232 | 1.17 µs | 175 µs | 658 µs | 1.52 ms | 2.75 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,122.3 124.4,119.7 160.5,125.0 196.7,120.4 232.9,124.0 269.1,118.9 305.3,120.7 341.5,122.6 377.6,122.3 413.8,117.8 450.0,122.3 450.0,26.0 413.8,32.2 377.6,44.6 341.5,50.8 305.3,59.8 269.1,64.5 232.9,73.5 196.7,77.9 160.5,87.7 124.4,91.4 88.2,96.2 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,122.3 124.4,119.7 160.5,125.0 196.7,120.4 232.9,124.0 269.1,118.9 305.3,120.7 341.5,122.6 377.6,122.3 413.8,117.8 450.0,122.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,107.1 124.4,106.2 160.5,102.5 196.7,97.4 232.9,95.0 269.1,87.8 305.3,84.2 341.5,82.5 377.6,73.4 413.8,64.4 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,105.8 124.4,98.2 160.5,95.4 196.7,86.6 232.9,89.9 269.1,77.7 305.3,72.1 341.5,64.4 377.6,58.2 413.8,54.6 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,97.7 160.5,90.7 196.7,81.0 232.9,81.0 269.1,71.6 305.3,64.4 341.5,56.3 377.6,48.5 413.8,40.2 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,96.2 124.4,91.4 160.5,87.7 196.7,77.9 232.9,73.5 269.1,64.5 305.3,59.8 341.5,50.8 377.6,44.6 413.8,32.2 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.46 µs | 2.54 µs | 2.81 µs | 2.5 µs |
| D38 | 4.04 ns | 2.12 µs | 3.24 µs | 3.74 µs | 4.28 µs |
| D57 | 2.81 ns | 3.84 µs | 7.38 µs | 7.3 µs | 11.7 µs |
| D76 | 2.08 ns | 5.23 µs | 8.89 µs | 11.9 µs | 14.1 µs |
| D115 | 8.1 ns | 13.7 µs | 20.4 µs | 24.5 µs | 33.1 µs |
| D153 | 7.42 ns | 15.7 µs | 14.5 µs | 25.2 µs | 43.6 µs |
| D230 | 19.5 ns | 21.5 µs | 41.9 µs | 62.3 µs | 95.5 µs |
| D307 | 24.5 ns | 38.7 µs | 72.1 µs | 117 µs | 168 µs |
| D462 | 39.8 ns | 35.8 µs | 118 µs | 197 µs | 317 µs |
| D616 | 37.5 ns | 104 µs | 225 µs | 431 µs | 567 µs |
| D924 | 35 ns | 228 µs | 315 µs | 864 µs | 1.59 ms |
| D1232 | 43.2 ns | 300 µs | 861 µs | 1.62 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,192.7 124.4,197.2 160.5,200.9 196.7,184.0 232.9,185.1 269.1,173.2 305.3,170.3 341.5,164.3 377.6,165.0 413.8,165.9 450.0,163.3 450.0,27.5 413.8,32.8 377.6,45.6 341.5,52.8 305.3,60.7 269.1,67.7 232.9,77.5 196.7,80.8 160.5,91.5 124.4,93.8 88.2,106.2 52.0,112.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,192.7 124.4,197.2 160.5,200.9 196.7,184.0 232.9,185.1 269.1,173.2 305.3,170.3 341.5,164.3 377.6,165.0 413.8,165.9 450.0,163.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.6 88.2,115.0 124.4,107.6 160.5,103.8 196.7,91.8 232.9,90.1 269.1,86.2 305.3,78.9 341.5,79.9 377.6,66.6 413.8,56.9 450.0,53.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,109.7 124.4,99.5 160.5,97.2 196.7,86.9 232.9,91.1 269.1,77.9 305.3,71.2 341.5,65.1 377.6,57.1 413.8,52.9 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,107.9 124.4,99.6 160.5,93.6 196.7,84.6 232.9,84.3 269.1,73.0 305.3,65.2 341.5,58.7 377.6,49.0 413.8,40.4 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.9 88.2,106.2 124.4,93.8 160.5,91.5 196.7,80.8 232.9,77.5 269.1,67.7 305.3,60.7 341.5,52.8 377.6,45.6 413.8,32.8 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.35 µs | 2.72 µs | 2.95 µs | 2.82 µs |
| D38 | 2.8 ns | 2.27 µs | 3.36 µs | 3.53 µs | 4.08 µs |
| D57 | 1.78 ns | 1.96 µs | 3.53 µs | 3.25 µs | 5.32 µs |
| D76 | 347 ns | 4.62 µs | 8.08 µs | 12.3 µs | 15.8 µs |
| D115 | 536 ns | 6.79 µs | 17.3 µs | 29.6 µs | 38.3 µs |
| D153 | 389 ns | 8.35 µs | 10.4 µs | 29.8 µs | 54.9 µs |
| D230 | 659 ns | 15.5 µs | 38.6 µs | 64.6 µs | 114 µs |
| D307 | 565 ns | 22.5 µs | 54 µs | 115 µs | 169 µs |
| D462 | 474 ns | 23.9 µs | 105 µs | 211 µs | 321 µs |
| D616 | 539 ns | 56.2 µs | 191 µs | 422 µs | 583 µs |
| D924 | 821 ns | 116 µs | 259 µs | 828 µs | 1.57 ms |
| D1232 | 571 ns | 162 µs | 626 µs | 1.46 ms | 2.62 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,197.2 124.4,202.9 160.5,137.4 196.7,132.0 232.9,136.0 269.1,129.5 305.3,131.4 341.5,133.5 377.6,132.0 413.8,126.7 450.0,131.2 450.0,26.6 413.8,33.0 377.6,45.3 341.5,52.7 305.3,60.7 269.1,65.5 232.9,74.6 196.7,79.1 160.5,90.0 124.4,103.5 88.2,106.9 52.0,111.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,197.2 124.4,202.9 160.5,137.4 196.7,132.0 232.9,136.0 269.1,129.5 305.3,131.4 341.5,133.5 377.6,132.0 413.8,126.7 450.0,131.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.5 88.2,114.1 124.4,116.0 160.5,105.3 196.7,100.5 232.9,98.0 269.1,90.3 305.3,85.6 341.5,84.9 377.6,74.3 413.8,65.3 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.8 88.2,109.3 124.4,108.6 160.5,98.4 196.7,88.9 232.9,95.2 269.1,79.0 305.3,74.8 341.5,66.6 377.6,59.1 413.8,55.3 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,108.6 124.4,109.7 160.5,93.2 196.7,82.2 232.9,82.2 269.1,72.6 305.3,65.4 341.5,57.9 377.6,49.3 413.8,40.9 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,106.9 124.4,103.5 160.5,90.0 196.7,79.1 232.9,74.6 269.1,65.5 305.3,60.7 341.5,52.7 377.6,45.3 413.8,33.0 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.83 ns | 1.47 µs | 2.87 µs | 3.16 µs | 2.9 µs |
| D38 | 4.04 ns | 2.39 µs | 3.74 µs | 4.18 µs | 4.89 µs |
| D57 | 523 ns | 3.9 µs | 7.07 µs | 7.06 µs | 11.6 µs |
| D76 | 278 ns | 5.2 µs | 8.42 µs | 11.6 µs | 14.4 µs |
| D115 | 894 ns | 13.3 µs | 20.1 µs | 25.4 µs | 33.6 µs |
| D153 | 769 ns | 15 µs | 14.4 µs | 25.4 µs | 49.9 µs |
| D230 | 1.43 µs | 20.3 µs | 41.3 µs | 71.4 µs | 118 µs |
| D307 | 1.94 µs | 40.7 µs | 84.6 µs | 147 µs | 239 µs |
| D462 | 1.51 µs | 36.4 µs | 145 µs | 288 µs | 476 µs |
| D616 | 2.59 µs | 114 µs | 297 µs | 589 µs | 905 µs |
| D924 | 5.33 µs | 255 µs | 430 µs | 1.39 ms | 2.66 ms |
| D1232 | 3.78 µs | 365 µs | 1.22 ms | 2.69 ms | 4.54 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.3 88.2,192.7 124.4,132.3 160.5,140.2 196.7,125.7 232.9,127.6 269.1,119.8 305.3,116.1 341.5,119.1 377.6,112.5 413.8,103.5 450.0,107.8 450.0,19.8 413.8,26.4 377.6,39.8 341.5,47.8 305.3,56.3 269.1,65.1 232.9,75.8 196.7,80.7 160.5,91.2 124.4,93.9 88.2,104.6 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.3 88.2,192.7 124.4,132.3 160.5,140.2 196.7,125.7 232.9,127.6 269.1,119.8 305.3,116.1 341.5,119.1 377.6,112.5 413.8,103.5 450.0,107.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.5 88.2,113.5 124.4,107.4 160.5,103.8 196.7,92.2 232.9,90.7 269.1,86.9 305.3,78.3 341.5,79.7 377.6,65.5 413.8,55.5 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.9 124.4,100.0 160.5,97.9 196.7,87.0 232.9,91.2 269.1,78.1 305.3,69.2 341.5,62.6 377.6,53.6 413.8,49.0 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,100.0 160.5,93.9 196.7,84.2 232.9,84.1 269.1,71.3 305.3,62.3 341.5,54.0 377.6,45.1 413.8,34.5 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,104.6 124.4,93.9 160.5,91.2 196.7,80.7 232.9,75.8 269.1,65.1 305.3,56.3 341.5,47.8 377.6,39.8 413.8,26.4 450.0,19.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.04 µs | 2.53 µs | 2.62 µs | 2.48 µs |
| D38 | 4.98 ns | 2.18 µs | 3.22 µs | 3.5 µs | 3.85 µs |
| D57 | 2.81 ns | 2.59 µs | 4.69 µs | 4.34 µs | 9.17 µs |
| D76 | 2.55 ns | 3.34 µs | 5.61 µs | 7.53 µs | 9.34 µs |
| D115 | 11.6 ns | 5.16 µs | 10.2 µs | 13.8 µs | 18.9 µs |
| D153 | 14.1 ns | 6.07 µs | 5.99 µs | 14 µs | 28.9 µs |
| D230 | 40.8 ns | 8.95 µs | 18.4 µs | 36.4 µs | 70.1 µs |
| D307 | 64.7 ns | 12 µs | 27.3 µs | 70.6 µs | 112 µs |
| D462 | 97.7 ns | 9.86 µs | 61.7 µs | 135 µs | 220 µs |
| D616 | 102 ns | 29.8 µs | 122 µs | 270 µs | 414 µs |
| D924 | 178 ns | 71.4 µs | 186 µs | 578 µs | 1.12 ms |
| D1232 | 191 ns | 103 µs | 427 µs | 1.05 ms | 1.93 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.1 124.4,197.2 160.5,198.4 196.7,179.6 232.9,177.2 269.1,164.0 305.3,158.3 341.5,153.1 377.6,152.6 413.8,145.7 450.0,144.8 450.0,30.4 413.8,37.1 377.6,49.5 341.5,57.4 305.3,65.7 269.1,71.5 232.9,82.5 196.7,87.8 160.5,96.6 124.4,96.8 88.2,107.5 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.1 124.4,197.2 160.5,198.4 196.7,179.6 232.9,177.2 269.1,164.0 305.3,158.3 341.5,153.1 377.6,152.6 413.8,145.7 450.0,144.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.8 88.2,114.6 124.4,112.5 160.5,109.3 196.7,103.9 232.9,101.9 269.1,97.1 305.3,93.4 341.5,95.9 377.6,82.2 413.8,71.3 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,109.8 124.4,105.1 160.5,102.9 196.7,95.5 232.9,102.1 269.1,88.1 305.3,83.2 341.5,73.1 377.6,64.6 413.8,59.4 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,108.8 124.4,106.1 160.5,99.2 196.7,91.7 232.9,91.5 269.1,79.7 305.3,71.5 341.5,63.5 377.6,54.8 413.8,45.4 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,107.5 124.4,96.8 160.5,96.6 196.7,87.8 232.9,82.5 269.1,71.5 305.3,65.7 341.5,57.4 377.6,49.5 413.8,37.1 450.0,30.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.52 µs | 3.31 µs | 3.54 µs | 3.28 µs |
| D38 | 4.04 ns | 2.89 µs | 4.3 µs | 4.41 µs | 4.87 µs |
| D57 | 3.17 ns | 4.02 µs | 7.06 µs | 6.53 µs | 10.3 µs |
| D76 | 3.19 ns | 5.12 µs | 7.82 µs | 10.5 µs | 12.3 µs |
| D115 | 9.97 ns | 13 µs | 12.9 µs | 22.3 µs | 26.5 µs |
| D153 | 13.4 ns | 8.25 µs | 9.85 µs | 17.4 µs | 35.6 µs |
| D230 | 40.8 ns | 12.9 µs | 23.1 µs | 43.3 µs | 80.4 µs |
| D307 | 65.4 ns | 16.9 µs | 53.5 µs | 81.1 µs | 120 µs |
| D462 | 92.6 ns | 15.9 µs | 82.1 µs | 157 µs | 232 µs |
| D616 | 102 ns | 36.2 µs | 135 µs | 289 µs | 406 µs |
| D924 | 200 ns | 82.7 µs | 185 µs | 570 µs | 993 µs |
| D1232 | 184 ns | 113 µs | 415 µs | 924 µs | 2.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,192.7 124.4,195.7 160.5,195.6 196.7,181.5 232.9,177.8 269.1,164.0 305.3,158.1 341.5,153.8 377.6,152.7 413.8,144.3 450.0,145.3 450.0,28.8 413.8,38.7 377.6,49.8 341.5,56.7 305.3,64.9 269.1,69.8 232.9,80.0 196.7,83.6 160.5,93.1 124.4,95.4 88.2,104.6 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,192.7 124.4,195.7 160.5,195.6 196.7,181.5 232.9,177.8 269.1,164.0 305.3,158.1 341.5,153.8 377.6,152.7 413.8,144.3 450.0,145.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.1 88.2,111.1 124.4,107.0 160.5,104.0 196.7,92.5 232.9,98.1 269.1,92.6 305.3,89.2 341.5,90.0 377.6,79.7 413.8,69.5 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,106.2 124.4,100.0 160.5,98.8 196.7,92.6 232.9,95.9 269.1,85.3 305.3,74.9 341.5,69.6 377.6,63.5 413.8,59.5 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,105.9 124.4,101.0 160.5,95.1 196.7,85.8 232.9,88.9 269.1,77.5 305.3,69.7 341.5,61.6 377.6,54.0 413.8,45.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,104.6 124.4,95.4 160.5,93.1 196.7,83.6 232.9,80.0 269.1,69.8 305.3,64.9 341.5,56.7 377.6,49.8 413.8,38.7 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 945 ns | 2.36 µs | 2.58 µs | 2.43 µs |
| D38 | 4.36 ns | 2.03 µs | 3.06 µs | 3.34 µs | 3.83 µs |
| D57 | 2.81 ns | 2.4 µs | 4.42 µs | 4.3 µs | 9.18 µs |
| D76 | 2.79 ns | 3.16 µs | 5.34 µs | 7.37 µs | 8.96 µs |
| D115 | 11.6 ns | 4.92 µs | 9.99 µs | 13.4 µs | 18.1 µs |
| D153 | 14.1 ns | 5.63 µs | 6.17 µs | 13.6 µs | 28.3 µs |
| D230 | 40.8 ns | 8.83 µs | 18.8 µs | 36.2 µs | 67.5 µs |
| D307 | 60.8 ns | 11.7 µs | 26.4 µs | 66.9 µs | 109 µs |
| D462 | 83.8 ns | 9.77 µs | 59.4 µs | 133 µs | 218 µs |
| D616 | 153 ns | 29 µs | 120 µs | 265 µs | 410 µs |
| D924 | 188 ns | 68.9 µs | 167 µs | 573 µs | 1.12 ms |
| D1232 | 180 ns | 101 µs | 428 µs | 1.04 ms | 1.91 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,197.3 196.7,179.5 232.9,177.2 269.1,164.0 305.3,159.0 341.5,155.1 377.6,147.6 413.8,145.0 450.0,145.6 450.0,30.5 413.8,37.2 377.6,49.6 341.5,57.5 305.3,66.1 269.1,72.0 232.9,82.8 196.7,88.3 160.5,97.1 124.4,96.8 88.2,107.6 52.0,113.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,197.3 196.7,179.5 232.9,177.2 269.1,164.0 305.3,159.0 341.5,155.1 377.6,147.6 413.8,145.0 450.0,145.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.0 88.2,115.5 124.4,113.4 160.5,110.0 196.7,104.5 232.9,102.8 269.1,97.3 305.3,93.8 341.5,96.0 377.6,82.5 413.8,71.8 450.0,67.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.6 88.2,110.4 124.4,105.8 160.5,103.5 196.7,95.7 232.9,101.7 269.1,87.9 305.3,83.7 341.5,73.6 377.6,64.9 413.8,60.8 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,109.3 124.4,106.2 160.5,99.5 196.7,92.1 232.9,91.9 269.1,79.8 305.3,72.1 341.5,63.6 377.6,55.1 413.8,45.5 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.3 88.2,107.6 124.4,96.8 160.5,97.1 196.7,88.3 232.9,82.8 269.1,72.0 305.3,66.1 341.5,57.5 377.6,49.6 413.8,37.2 450.0,30.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.94 ns | 1.52 µs | 3.32 µs | 3.55 µs | 3.29 µs |
| D38 | 4.04 ns | 2.89 µs | 4.3 µs | 4.45 µs | 4.9 µs |
| D57 | 12.3 ns | 4.05 µs | 7.08 µs | 6.55 µs | 10.3 µs |
| D76 | 8.21 ns | 5.11 µs | 7.84 µs | 10.6 µs | 12.3 µs |
| D115 | 10.3 ns | 13.1 µs | 13.3 µs | 21.9 µs | 26.7 µs |
| D153 | 13.1 ns | 8.33 µs | 9.91 µs | 17.3 µs | 35.5 µs |
| D230 | 40 ns | 13 µs | 23.1 µs | 43.4 µs | 80.5 µs |
| D307 | 60.3 ns | 17.4 µs | 53 µs | 81.4 µs | 119 µs |
| D462 | 88.2 ns | 15.5 µs | 82.2 µs | 157 µs | 232 µs |
| D616 | 155 ns | 36.3 µs | 135 µs | 291 µs | 406 µs |
| D924 | 205 ns | 82.6 µs | 181 µs | 570 µs | 992 µs |
| D1232 | 176 ns | 114 µs | 416 µs | 925 µs | 2.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.0 88.2,192.7 124.4,178.9 160.5,183.9 196.7,181.1 232.9,178.1 269.1,164.2 305.3,159.1 341.5,154.4 377.6,147.5 413.8,144.0 450.0,145.8 450.0,28.8 413.8,38.7 377.6,49.8 341.5,56.7 305.3,65.0 269.1,69.8 232.9,80.0 196.7,83.5 160.5,93.1 124.4,95.4 88.2,104.6 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.0 88.2,192.7 124.4,178.9 160.5,183.9 196.7,181.1 232.9,178.1 269.1,164.2 305.3,159.1 341.5,154.4 377.6,147.5 413.8,144.0 450.0,145.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.1 88.2,111.1 124.4,106.9 160.5,104.0 196.7,92.4 232.9,98.0 269.1,92.5 305.3,88.9 341.5,90.3 377.6,79.7 413.8,69.5 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,106.2 124.4,100.0 160.5,98.7 196.7,92.1 232.9,95.8 269.1,85.3 305.3,75.0 341.5,69.6 377.6,63.4 413.8,59.7 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,105.8 124.4,101.0 160.5,95.0 196.7,86.0 232.9,88.9 269.1,77.5 305.3,69.7 341.5,61.5 377.6,53.9 413.8,45.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,104.6 124.4,95.4 160.5,93.1 196.7,83.5 232.9,80.0 269.1,69.8 305.3,65.0 341.5,56.7 377.6,49.8 413.8,38.7 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.9 µs | 3.99 µs | 4.24 µs | 4.04 µs |
| D38 | 4.36 ns | 3.48 µs | 5.14 µs | 5.58 µs | 6.25 µs |
| D57 | 3.16 ns | 3.23 µs | 5.99 µs | 5.73 µs | 9.24 µs |
| D76 | 3.08 ns | 4.11 µs | 7.15 µs | 9.58 µs | 11.2 µs |
| D115 | 10.6 ns | 6.42 µs | 12.9 µs | 16.4 µs | 21.5 µs |
| D153 | 13.6 ns | 7.46 µs | 7.71 µs | 16.4 µs | 33.1 µs |
| D230 | 41.2 ns | 11.1 µs | 22.3 µs | 41.2 µs | 75.5 µs |
| D307 | 58.8 ns | 14.3 µs | 31.2 µs | 75.5 µs | 121 µs |
| D462 | 86.2 ns | 12.2 µs | 67.6 µs | 147 µs | 234 µs |
| D616 | 87.4 ns | 34.4 µs | 134 µs | 290 µs | 439 µs |
| D924 | 158 ns | 77.9 µs | 185 µs | 622 µs | 1.2 ms |
| D1232 | 187 ns | 112 µs | 457 µs | 1.11 ms | 2.02 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,195.7 160.5,196.1 196.7,180.7 232.9,177.6 269.1,163.9 305.3,159.4 341.5,154.7 377.6,154.5 413.8,147.2 450.0,145.1 450.0,29.8 413.8,36.3 377.6,48.8 341.5,56.6 305.3,64.8 269.1,70.6 232.9,80.9 196.7,86.2 160.5,94.4 124.4,96.7 88.2,101.5 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,195.7 160.5,196.1 196.7,180.7 232.9,177.6 269.1,163.9 305.3,159.4 341.5,154.7 377.6,154.5 413.8,147.2 450.0,145.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.3 88.2,108.8 124.4,109.7 160.5,106.7 196.7,101.2 232.9,99.4 269.1,94.5 305.3,91.3 341.5,93.3 377.6,80.4 413.8,70.2 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,104.0 124.4,102.1 160.5,99.9 196.7,92.6 232.9,98.9 269.1,85.7 305.3,81.6 341.5,72.0 377.6,63.5 413.8,59.5 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.4 88.2,103.0 124.4,102.6 160.5,96.2 196.7,89.6 232.9,89.6 269.1,78.2 305.3,70.6 341.5,62.4 377.6,53.9 413.8,44.5 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,101.5 124.4,96.7 160.5,94.4 196.7,86.2 232.9,80.9 269.1,70.6 305.3,64.8 341.5,56.6 377.6,48.8 413.8,36.3 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.81 ns | 1.54 µs | 3.48 µs | 3.72 µs | 3.44 µs |
| D38 | 4.04 ns | 3.03 µs | 4.37 µs | 4.43 µs | 4.91 µs |
| D57 | 2.81 µs | 4.18 µs | 7.47 µs | 6.81 µs | 10.9 µs |
| D76 | 1.86 µs | 5.26 µs | 8.24 µs | 11 µs | 12.8 µs |
| D115 | 5.25 µs | 13.7 µs | 13.5 µs | 22.5 µs | 27.4 µs |
| D153 | 2.12 µs | 8.73 µs | 10.1 µs | 17.9 µs | 36.5 µs |
| D230 | 3.07 µs | 13.5 µs | 23.7 µs | 44.9 µs | 81.5 µs |
| D307 | 2.91 µs | 17.5 µs | 55.3 µs | 82.8 µs | 121 µs |
| D462 | 2.55 µs | 16.2 µs | 84 µs | 159 µs | 235 µs |
| D616 | 2.72 µs | 37.4 µs | 137 µs | 294 µs | 410 µs |
| D924 | 3.77 µs | 83.9 µs | 188 µs | 573 µs | 1 ms |
| D1232 | 2.46 µs | 115 µs | 420 µs | 930 µs | 2.21 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.4 88.2,192.7 124.4,111.5 160.5,116.6 196.7,103.7 232.9,114.9 269.1,110.4 305.3,111.0 341.5,112.7 377.6,111.9 413.8,107.8 450.0,113.1 450.0,28.8 413.8,38.5 377.6,49.6 341.5,56.5 305.3,64.8 269.1,69.7 232.9,79.7 196.7,83.2 160.5,92.6 124.4,94.7 88.2,104.5 52.0,108.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.4 88.2,192.7 124.4,111.5 160.5,116.6 196.7,103.7 232.9,114.9 269.1,110.4 305.3,111.0 341.5,112.7 377.6,111.9 413.8,107.8 450.0,113.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.9 88.2,110.5 124.4,106.5 160.5,103.7 196.7,91.8 232.9,97.4 269.1,92.0 305.3,88.8 341.5,89.7 377.6,79.3 413.8,69.3 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.0 124.4,99.3 160.5,98.1 196.7,92.0 232.9,95.6 269.1,85.0 305.3,74.5 341.5,69.3 377.6,63.2 413.8,59.3 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,105.8 124.4,100.5 160.5,94.5 196.7,85.7 232.9,88.5 269.1,77.1 305.3,69.5 341.5,61.4 377.6,53.8 413.8,45.5 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.5 124.4,94.7 160.5,92.6 196.7,83.2 232.9,79.7 269.1,69.7 305.3,64.8 341.5,56.5 377.6,49.6 413.8,38.5 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 109 ns | 158 ns | 159 ns | 137 ns |
| D38 | 4.36 ns | 131 ns | 175 ns | 183 ns | 183 ns |
| D57 | 210 ns | 205 ns | 326 ns | 253 ns | 409 ns |
| D76 | 112 ns | 254 ns | 333 ns | 411 ns | 437 ns |
| D115 | 413 ns | 659 ns | 811 ns | 808 ns | 854 ns |
| D153 | 265 ns | 687 ns | 419 ns | 595 ns | 975 ns |
| D230 | 567 ns | 770 ns | 1.01 µs | 1.3 µs | 1.83 µs |
| D307 | 835 ns | 1.24 µs | 1.71 µs | 2.15 µs | 2.68 µs |
| D462 | 722 ns | 910 ns | 2.24 µs | 3.09 µs | 3.95 µs |
| D616 | 898 ns | 1.89 µs | 2.97 µs | 4.53 µs | 5.57 µs |
| D924 | 1.67 µs | 2.85 µs | 2.96 µs | 7.17 µs | 11.3 µs |
| D1232 | 1.28 µs | 3.34 µs | 7.19 µs | 11.3 µs | 23.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,117.1 160.5,128.0 196.7,105.4 232.9,113.1 269.1,99.9 305.3,93.1 341.5,95.7 377.6,91.9 413.8,81.1 450.0,85.7 450.0,35.0 413.8,47.8 377.6,60.2 341.5,66.1 305.3,72.9 269.1,79.5 232.9,90.4 196.7,92.7 160.5,104.4 124.4,105.5 88.2,119.5 52.0,124.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,117.1 160.5,128.0 196.7,105.4 232.9,113.1 269.1,99.9 305.3,93.1 341.5,95.7 377.6,91.9 413.8,81.1 450.0,85.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,128.5 88.2,125.3 124.4,117.6 160.5,113.8 196.7,97.2 232.9,96.5 269.1,94.5 305.3,86.3 341.5,91.6 377.6,78.9 413.8,71.8 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.1 88.2,120.2 124.4,109.5 160.5,109.1 196.7,93.6 232.9,105.1 269.1,89.9 305.3,80.7 341.5,76.0 377.6,71.1 413.8,71.1 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,119.5 124.4,113.9 160.5,105.4 196.7,93.7 232.9,99.0 269.1,85.4 305.3,76.7 341.5,70.4 377.6,63.8 413.8,55.8 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.5 88.2,119.5 124.4,105.5 160.5,104.4 196.7,92.7 232.9,90.4 269.1,79.5 305.3,72.9 341.5,66.1 377.6,60.2 413.8,47.8 450.0,35.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 139 ns | 199 ns | 197 ns | 180 ns |
| D38 | 4.36 ns | 172 ns | 217 ns | 199 ns | 201 ns |
| D57 | 263 ns | 234 ns | 386 ns | 304 ns | 498 ns |
| D76 | 150 ns | 312 ns | 422 ns | 503 ns | 534 ns |
| D115 | 548 ns | 751 ns | 927 ns | 944 ns | 975 ns |
| D153 | 346 ns | 810 ns | 483 ns | 670 ns | 1.13 µs |
| D230 | 778 ns | 886 ns | 1.14 µs | 1.5 µs | 1.98 µs |
| D307 | 1.13 µs | 1.49 µs | 2.01 µs | 2.47 µs | 2.99 µs |
| D462 | 930 ns | 1.01 µs | 2.52 µs | 3.43 µs | 4.29 µs |
| D616 | 1.12 µs | 2.11 µs | 3.35 µs | 4.93 µs | 5.96 µs |
| D924 | 2.12 µs | 3.27 µs | 3.38 µs | 7.69 µs | 12 µs |
| D1232 | 1.58 µs | 3.79 µs | 7.59 µs | 12 µs | 24.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,113.2 160.5,122.9 196.7,100.4 232.9,108.4 269.1,94.4 305.3,87.8 341.5,91.3 377.6,88.0 413.8,76.9 450.0,82.1 450.0,34.6 413.8,46.9 377.6,59.0 341.5,64.7 305.3,71.0 269.1,78.1 232.9,87.8 196.7,90.4 160.5,100.9 124.4,102.1 88.2,117.9 52.0,119.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,113.2 160.5,122.9 196.7,100.4 232.9,108.4 269.1,94.4 305.3,87.8 341.5,91.3 377.6,88.0 413.8,76.9 450.0,82.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.3 88.2,120.6 124.4,115.2 160.5,110.3 196.7,95.0 232.9,93.7 269.1,92.1 305.3,83.1 341.5,89.7 377.6,77.0 413.8,69.4 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.1 88.2,116.6 124.4,106.5 160.5,105.0 196.7,91.3 232.9,102.6 269.1,87.8 305.3,77.9 341.5,73.9 377.6,69.0 413.8,68.9 450.0,54.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,118.1 124.4,110.7 160.5,101.9 196.7,91.0 232.9,97.0 269.1,83.0 305.3,74.3 341.5,68.6 377.6,62.3 413.8,54.6 450.0,46.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.8 88.2,117.9 124.4,102.1 160.5,100.9 196.7,90.4 232.9,87.8 269.1,78.1 305.3,71.0 341.5,64.7 377.6,59.0 413.8,46.9 450.0,34.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
