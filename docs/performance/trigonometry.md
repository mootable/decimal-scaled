# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.95 ns | 2 µs | 3.48 µs | 3.49 µs | 3.31 µs |
| D38 | 1.1 µs | 4.63 µs | 4.55 µs | 8.04 µs | 9.12 µs |
| D57 | 1.45 µs | 5.8 µs | 8.71 µs | 11.2 µs | 11 µs |
| D76 | 1.45 µs | 7.17 µs | 9.97 µs | 14.5 µs | 14.8 µs |
| D115 | 1.21 µs | 8.39 µs | 19.8 µs | 33.1 µs | 42.5 µs |
| D153 | 1.58 µs | 8.25 µs | 18.6 µs | 37.4 µs | 65.3 µs |
| D230 | 1.35 µs | 20.5 µs | 45.4 µs | 49.9 µs | 109 µs |
| D307 | 1.56 µs | 21.9 µs | 41.8 µs | 132 µs | 161 µs |
| D462 | 1.57 µs | 37.8 µs | 124 µs | 200 µs | 375 µs |
| D616 | 1.55 µs | 49.7 µs | 207 µs | 445 µs | 707 µs |
| D924 | 1.66 µs | 83.7 µs | 358 µs | 742 µs | 1.32 ms |
| D1232 | 1.99 µs | 136 µs | 715 µs | 1.31 ms | 3.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.3 88.2,123.1 124.4,119.7 160.5,119.6 196.7,122.0 232.9,118.6 269.1,120.5 305.3,118.8 341.5,118.7 377.6,118.9 413.8,118.0 450.0,115.7 450.0,23.7 413.8,35.1 377.6,42.9 341.5,50.7 305.3,61.2 269.1,66.1 232.9,72.4 196.7,77.8 160.5,90.8 124.4,94.5 88.2,96.9 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.3 88.2,123.1 124.4,119.7 160.5,119.6 196.7,122.0 232.9,118.6 269.1,120.5 305.3,118.8 341.5,118.7 377.6,118.9 413.8,118.0 450.0,115.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.3 124.4,102.5 160.5,99.8 196.7,97.9 232.9,98.1 269.1,86.8 305.3,86.0 341.5,79.2 377.6,75.8 413.8,69.4 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,105.5 124.4,97.4 160.5,95.8 196.7,87.2 232.9,88.0 269.1,76.9 305.3,78.0 341.5,64.5 377.6,58.1 413.8,51.3 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,94.3 160.5,91.1 196.7,80.9 232.9,79.4 269.1,75.8 305.3,63.7 341.5,58.5 377.6,48.6 413.8,42.3 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,96.9 124.4,94.5 160.5,90.8 196.7,77.8 232.9,72.4 269.1,66.1 305.3,61.2 341.5,50.7 377.6,42.9 413.8,35.1 450.0,23.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.38 µs | 3.61 µs | 6.06 µs | 6.38 µs | 5.76 µs |
| D38 | 2.74 µs | 5.83 µs | 7.55 µs | 8.39 µs | 9.26 µs |
| D57 | 3.46 µs | 4.32 µs | 5.46 µs | 6.64 µs | 6 µs |
| D76 | 3.41 µs | 5.1 µs | 5.91 µs | 7.79 µs | 7.49 µs |
| D115 | 5.1 µs | 8.54 µs | 12.3 µs | 16.6 µs | 21.7 µs |
| D153 | 6.68 µs | 8.64 µs | 11.1 µs | 17.9 µs | 33.9 µs |
| D230 | 7.51 µs | 16 µs | 28.7 µs | 28.2 µs | 60.4 µs |
| D307 | 13.3 µs | 22.2 µs | 31.3 µs | 89.4 µs | 114 µs |
| D462 | 12.5 µs | 30.6 µs | 84.9 µs | 134 µs | 262 µs |
| D616 | 22 µs | 53.4 µs | 170 µs | 329 µs | 551 µs |
| D924 | 32.5 µs | 100 µs | 317 µs | 633 µs | 1.13 ms |
| D1232 | 44 µs | 161 µs | 734 µs | 1.23 ms | 2.94 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.5 88.2,188.1 124.4,183.1 160.5,183.3 196.7,174.6 232.9,168.8 269.1,166.2 305.3,153.9 341.5,155.1 377.6,142.9 413.8,134.4 450.0,127.8 450.0,36.6 413.8,57.3 377.6,73.0 341.5,89.1 305.3,107.1 269.1,120.9 232.9,133.5 196.7,143.2 160.5,166.3 124.4,171.1 88.2,161.7 52.0,172.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.5 88.2,188.1 124.4,183.1 160.5,183.3 196.7,174.6 232.9,168.8 269.1,166.2 305.3,153.9 341.5,155.1 377.6,142.9 413.8,134.4 450.0,127.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.1 88.2,171.7 124.4,178.2 160.5,174.6 196.7,163.4 232.9,163.2 269.1,149.8 305.3,142.7 341.5,135.7 377.6,123.6 413.8,110.0 450.0,99.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.9 88.2,166.1 124.4,173.2 160.5,171.4 196.7,155.5 232.9,157.7 269.1,137.1 305.3,135.2 341.5,113.6 377.6,98.5 413.8,85.0 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.8 88.2,163.8 124.4,168.9 160.5,165.4 196.7,148.9 232.9,147.4 269.1,137.5 305.3,112.4 341.5,103.6 377.6,84.1 413.8,69.9 450.0,55.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.0 88.2,161.7 124.4,171.1 160.5,166.3 196.7,143.2 232.9,133.5 269.1,120.9 305.3,107.1 341.5,89.1 377.6,73.0 413.8,57.3 450.0,36.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 1.99 µs | 3.48 µs | 3.48 µs | 3.28 µs |
| D38 | 1.04 µs | 4.63 µs | 4.48 µs | 7.98 µs | 9.07 µs |
| D57 | 1.35 µs | 5.72 µs | 8.66 µs | 11.1 µs | 11 µs |
| D76 | 1.33 µs | 7.14 µs | 9.93 µs | 14.5 µs | 14.7 µs |
| D115 | 1.1 µs | 8.33 µs | 20 µs | 32.8 µs | 42.7 µs |
| D153 | 1.48 µs | 8.23 µs | 18.6 µs | 37.1 µs | 65.5 µs |
| D230 | 1.21 µs | 20 µs | 45.6 µs | 47.4 µs | 107 µs |
| D307 | 1.46 µs | 23 µs | 42.2 µs | 132 µs | 159 µs |
| D462 | 1.43 µs | 37.5 µs | 123 µs | 199 µs | 375 µs |
| D616 | 1.42 µs | 50.1 µs | 207 µs | 445 µs | 707 µs |
| D924 | 1.54 µs | 90.6 µs | 356 µs | 746 µs | 1.32 ms |
| D1232 | 1.87 µs | 133 µs | 715 µs | 1.3 ms | 3.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,123.8 124.4,120.6 160.5,120.7 196.7,123.1 232.9,119.4 269.1,121.9 305.3,119.6 341.5,119.9 377.6,119.9 413.8,118.9 450.0,116.5 450.0,23.7 413.8,35.1 377.6,42.9 341.5,50.7 305.3,61.4 269.1,66.3 232.9,72.4 196.7,77.7 160.5,90.9 124.4,94.5 88.2,96.9 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,123.8 124.4,120.6 160.5,120.7 196.7,123.1 232.9,119.4 269.1,121.9 305.3,119.6 341.5,119.9 377.6,119.9 413.8,118.9 450.0,116.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.3 124.4,102.6 160.5,99.9 196.7,98.0 232.9,98.1 269.1,87.1 305.3,85.4 341.5,79.3 377.6,75.7 413.8,68.4 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,105.7 124.4,97.5 160.5,95.8 196.7,87.1 232.9,88.0 269.1,76.9 305.3,77.9 341.5,64.5 377.6,58.1 413.8,51.4 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.5 124.4,94.4 160.5,91.1 196.7,81.0 232.9,79.5 269.1,76.4 305.3,63.7 341.5,58.6 377.6,48.6 413.8,42.2 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,96.9 124.4,94.5 160.5,90.9 196.7,77.7 232.9,72.4 269.1,66.3 305.3,61.4 341.5,50.7 377.6,42.9 413.8,35.1 450.0,23.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.74 µs | 1.71 µs | 2.67 µs | 2.81 µs | 2.5 µs |
| D38 | 1.39 µs | 2.52 µs | 3.29 µs | 3.75 µs | 4.12 µs |
| D57 | 4.49 µs | 5.3 µs | 7.8 µs | 9.54 µs | 9.39 µs |
| D76 | 4.47 µs | 6.75 µs | 8.61 µs | 11.6 µs | 11.2 µs |
| D115 | 7.19 µs | 12.6 µs | 19.3 µs | 24.8 µs | 34.3 µs |
| D153 | 9 µs | 13.2 µs | 17.4 µs | 28.4 µs | 47.6 µs |
| D230 | 10.3 µs | 24.7 µs | 44.2 µs | 41.4 µs | 84.2 µs |
| D307 | 17.7 µs | 32.5 µs | 47.8 µs | 123 µs | 144 µs |
| D462 | 16.6 µs | 46 µs | 118 µs | 164 µs | 315 µs |
| D616 | 29.6 µs | 76.7 µs | 227 µs | 429 µs | 668 µs |
| D924 | 42.7 µs | 159 µs | 406 µs | 742 µs | 1.26 ms |
| D1232 | 58.5 µs | 220 µs | 933 µs | 1.39 ms | 2.88 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.0 88.2,202.9 124.4,177.4 160.5,177.5 196.7,167.2 232.9,162.3 269.1,159.4 305.3,147.7 341.5,149.0 377.6,136.5 413.8,128.5 450.0,121.6 450.0,37.0 413.8,55.1 377.6,68.8 341.5,85.1 305.3,102.1 269.1,113.7 232.9,126.1 196.7,133.2 160.5,157.5 124.4,161.4 88.2,179.3 52.0,190.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.0 88.2,202.9 124.4,177.4 160.5,177.5 196.7,167.2 232.9,162.3 269.1,159.4 305.3,147.7 341.5,149.0 377.6,136.5 413.8,128.5 450.0,121.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.3 88.2,189.9 124.4,173.8 160.5,168.5 196.7,155.0 232.9,153.9 269.1,140.3 305.3,134.4 341.5,126.8 377.6,115.8 413.8,99.9 450.0,92.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.7 88.2,184.1 124.4,165.4 160.5,163.2 196.7,145.7 232.9,147.9 269.1,127.7 305.3,126.0 341.5,106.4 377.6,92.2 413.8,79.5 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,181.3 124.4,161.0 160.5,156.7 196.7,140.3 232.9,137.4 269.1,129.1 305.3,105.4 341.5,99.2 377.6,78.4 413.8,66.5 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.1 88.2,179.3 124.4,161.4 160.5,157.5 196.7,133.2 232.9,126.1 269.1,113.7 305.3,102.1 341.5,85.1 377.6,68.8 413.8,55.1 450.0,37.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.55 µs | 1.57 µs | 2.91 µs | 2.95 µs | 2.83 µs |
| D38 | 1.45 µs | 2.76 µs | 3.31 µs | 3.52 µs | 3.67 µs |
| D57 | 1.8 µs | 2.88 µs | 3.7 µs | 4.35 µs | 4.05 µs |
| D76 | 5.67 µs | 5.54 µs | 7.86 µs | 11.9 µs | 12.2 µs |
| D115 | 4.89 µs | 6.38 µs | 16.7 µs | 29.9 µs | 38.9 µs |
| D153 | 6.27 µs | 6.46 µs | 12.1 µs | 33.3 µs | 60.2 µs |
| D230 | 4.96 µs | 18.1 µs | 42 µs | 44.3 µs | 97.1 µs |
| D307 | 6.03 µs | 20.2 µs | 34.4 µs | 122 µs | 149 µs |
| D462 | 2.8 µs | 30.8 µs | 105 µs | 174 µs | 320 µs |
| D616 | 5.65 µs | 46.8 µs | 193 µs | 418 µs | 666 µs |
| D924 | 5.96 µs | 75 µs | 333 µs | 711 µs | 1.25 ms |
| D1232 | 6.04 µs | 124 µs | 677 µs | 1.25 ms | 3.17 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.7 88.2,201.9 124.4,197.3 160.5,172.3 196.7,175.5 232.9,170.1 269.1,175.2 305.3,171.0 341.5,187.6 377.6,172.4 413.8,171.2 450.0,171.0 450.0,35.0 413.8,55.2 377.6,68.8 341.5,84.7 305.3,101.4 269.1,110.6 232.9,121.0 196.7,130.5 160.5,155.7 124.4,179.7 88.2,181.8 52.0,187.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.7 88.2,201.9 124.4,197.3 160.5,172.3 196.7,175.5 232.9,170.1 269.1,175.2 305.3,171.0 341.5,187.6 377.6,172.4 413.8,171.2 450.0,171.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.1 88.2,187.9 124.4,187.1 160.5,172.8 196.7,169.8 232.9,169.5 269.1,147.1 305.3,144.8 341.5,135.5 377.6,126.5 413.8,116.2 450.0,105.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.8 88.2,184.0 124.4,181.6 160.5,165.2 196.7,148.9 232.9,155.9 269.1,128.8 305.3,133.2 341.5,109.0 377.6,95.8 413.8,83.9 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.5 88.2,182.7 124.4,178.1 160.5,156.1 196.7,136.2 232.9,133.9 269.1,127.7 305.3,105.7 341.5,98.0 377.6,78.9 413.8,67.4 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,181.8 124.4,179.7 160.5,155.7 196.7,130.5 232.9,121.0 269.1,110.6 305.3,101.4 341.5,84.7 377.6,68.8 413.8,55.2 450.0,35.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.41 ns | 1.74 µs | 2.98 µs | 3.15 µs | 2.89 µs |
| D38 | 3.54 ns | 2.85 µs | 3.75 µs | 4.16 µs | 4.66 µs |
| D57 | 452 ns | 5.69 µs | 7.59 µs | 9.34 µs | 8.91 µs |
| D76 | 450 ns | 6.72 µs | 8.37 µs | 11.3 µs | 11.3 µs |
| D115 | 722 ns | 11.9 µs | 17.9 µs | 25.4 µs | 34 µs |
| D153 | 1.1 µs | 11.9 µs | 16.6 µs | 27.7 µs | 55 µs |
| D230 | 1.36 µs | 23.2 µs | 45.3 µs | 47.5 µs | 103 µs |
| D307 | 2.12 µs | 33.7 µs | 51.9 µs | 153 µs | 200 µs |
| D462 | 2.06 µs | 47.6 µs | 145 µs | 239 µs | 473 µs |
| D616 | 3.52 µs | 85.4 µs | 296 µs | 591 µs | 1.01 ms |
| D924 | 5.03 µs | 155 µs | 563 µs | 1.17 ms | 2.11 ms |
| D1232 | 6.98 µs | 272 µs | 1.32 ms | 2.28 ms | 5.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,194.3 124.4,134.1 160.5,134.2 196.7,128.3 232.9,123.1 269.1,120.5 305.3,115.0 341.5,115.3 377.6,108.7 413.8,104.2 450.0,100.2 450.0,17.4 413.8,29.3 377.6,38.5 341.5,47.9 305.3,58.5 269.1,66.8 232.9,74.6 196.7,80.5 160.5,94.2 124.4,97.1 88.2,105.2 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,194.3 124.4,134.1 160.5,134.2 196.7,128.3 232.9,123.1 269.1,120.5 305.3,115.0 341.5,115.3 377.6,108.7 413.8,104.2 450.0,100.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,111.3 124.4,102.7 160.5,100.7 196.7,93.6 232.9,93.5 269.1,85.3 305.3,80.6 341.5,76.4 377.6,69.1 413.8,61.7 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,107.9 124.4,99.1 160.5,97.9 196.7,88.5 232.9,89.4 269.1,77.0 305.3,75.3 341.5,62.5 377.6,53.7 413.8,45.7 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.1 88.2,106.6 124.4,96.6 160.5,94.2 196.7,84.1 232.9,83.1 269.1,76.4 305.3,61.9 341.5,56.3 377.6,45.1 413.8,36.6 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,105.2 124.4,97.1 160.5,94.2 196.7,80.5 232.9,74.6 269.1,66.8 305.3,58.5 341.5,47.9 377.6,38.5 413.8,29.3 450.0,17.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.33 µs | 1.18 µs | 2.69 µs | 2.63 µs | 2.47 µs |
| D38 | 1.12 µs | 2.56 µs | 3.23 µs | 3.59 µs | 3.09 µs |
| D57 | 3.02 µs | 3.48 µs | 5.05 µs | 5.98 µs | 6.94 µs |
| D76 | 2.98 µs | 4.07 µs | 5.38 µs | 7.25 µs | 7.08 µs |
| D115 | 2.56 µs | 4.75 µs | 10.4 µs | 14 µs | 18.4 µs |
| D153 | 3.22 µs | 4.56 µs | 7.15 µs | 15.4 µs | 32.7 µs |
| D230 | 2.7 µs | 9.88 µs | 19.3 µs | 24.4 µs | 63 µs |
| D307 | 3.28 µs | 10.8 µs | 16.8 µs | 75 µs | 94.1 µs |
| D462 | 1.89 µs | 14.1 µs | 61.6 µs | 111 µs | 217 µs |
| D616 | 3.11 µs | 24.5 µs | 123 µs | 272 µs | 455 µs |
| D924 | 3.32 µs | 42.7 µs | 218 µs | 487 µs | 888 µs |
| D1232 | 3.73 µs | 78.1 µs | 461 µs | 889 µs | 2.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.8 88.2,207.6 124.4,186.0 160.5,186.3 196.7,189.5 232.9,184.6 269.1,188.5 305.3,184.2 341.5,196.2 377.6,185.4 413.8,183.9 450.0,181.4 450.0,41.6 413.8,62.6 377.6,77.1 341.5,93.1 305.3,111.3 269.1,120.0 232.9,134.3 196.7,146.7 160.5,167.5 124.4,167.9 88.2,185.5 52.0,190.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.8 88.2,207.6 124.4,186.0 160.5,186.3 196.7,189.5 232.9,184.6 269.1,188.5 305.3,184.2 341.5,196.2 377.6,185.4 413.8,183.9 450.0,181.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,206.4 88.2,189.6 124.4,182.9 160.5,179.5 196.7,176.2 232.9,177.1 269.1,160.3 305.3,158.4 341.5,152.6 377.6,140.6 413.8,128.5 450.0,115.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.5 88.2,184.5 124.4,174.8 160.5,173.4 196.7,159.2 232.9,167.3 269.1,145.7 305.3,148.7 341.5,120.5 377.6,105.5 413.8,93.1 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.0 88.2,182.3 124.4,171.2 160.5,167.0 196.7,152.6 232.9,150.7 269.1,140.6 305.3,116.2 341.5,107.8 377.6,88.3 413.8,75.6 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.3 88.2,185.5 124.4,167.9 160.5,167.5 196.7,146.7 232.9,134.3 269.1,120.0 305.3,111.3 341.5,93.1 377.6,77.1 413.8,62.6 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.71 µs | 1.72 µs | 3.63 µs | 3.55 µs | 3.27 µs |
| D38 | 1.37 µs | 3.35 µs | 4.34 µs | 4.47 µs | 4.16 µs |
| D57 | 5.67 µs | 5.48 µs | 7.56 µs | 8.48 µs | 8.29 µs |
| D76 | 5.72 µs | 6.37 µs | 7.62 µs | 10.2 µs | 9.94 µs |
| D115 | 11.1 µs | 12.3 µs | 12.2 µs | 22.1 µs | 27 µs |
| D153 | 6.34 µs | 6.55 µs | 11.8 µs | 20.1 µs | 39 µs |
| D230 | 5.16 µs | 14.1 µs | 24.6 µs | 29 µs | 69.4 µs |
| D307 | 6.54 µs | 14.7 µs | 32.1 µs | 86.2 µs | 103 µs |
| D462 | 6.06 µs | 20.9 µs | 81.8 µs | 133 µs | 233 µs |
| D616 | 6.18 µs | 29.3 µs | 135 µs | 288 µs | 449 µs |
| D924 | 6.43 µs | 52 µs | 231 µs | 479 µs | 780 µs |
| D1232 | 7.03 µs | 86.1 µs | 452 µs | 783 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.4 88.2,203.2 124.4,172.3 160.5,172.1 196.7,157.7 232.9,169.9 269.1,174.4 305.3,169.2 341.5,170.9 377.6,170.5 413.8,169.6 450.0,167.6 450.0,37.3 413.8,65.4 377.6,77.4 341.5,91.6 305.3,109.4 269.1,117.9 232.9,130.4 196.7,138.4 160.5,160.1 124.4,164.1 88.2,179.0 52.0,184.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.4 88.2,203.2 124.4,172.3 160.5,172.1 196.7,157.7 232.9,169.9 269.1,174.4 305.3,169.2 341.5,170.9 377.6,170.5 413.8,169.6 450.0,167.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.2 88.2,183.7 124.4,173.1 160.5,169.8 196.7,155.5 232.9,169.2 269.1,152.6 305.3,151.7 341.5,144.0 377.6,136.6 413.8,124.2 450.0,113.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,178.1 124.4,166.1 160.5,165.9 196.7,155.7 232.9,156.4 269.1,140.4 305.3,134.7 341.5,114.4 377.6,103.6 413.8,91.8 450.0,77.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.5 88.2,177.5 124.4,163.6 160.5,159.5 196.7,142.8 232.9,144.8 269.1,136.9 305.3,113.2 341.5,103.9 377.6,87.0 413.8,76.0 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.3 88.2,179.0 124.4,164.1 160.5,160.1 196.7,138.4 232.9,130.4 269.1,117.9 305.3,109.4 341.5,91.6 377.6,77.4 413.8,65.4 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.42 µs | 1.07 µs | 2.53 µs | 2.68 µs | 2.43 µs |
| D38 | 1.19 µs | 2.4 µs | 3.08 µs | 3.37 µs | 3.06 µs |
| D57 | 3.14 µs | 3.24 µs | 4.81 µs | 5.95 µs | 6.97 µs |
| D76 | 3.2 µs | 3.86 µs | 5.11 µs | 7.16 µs | 6.86 µs |
| D115 | 2.75 µs | 4.46 µs | 10.1 µs | 13.8 µs | 18.3 µs |
| D153 | 3.37 µs | 4.33 µs | 7.14 µs | 14.8 µs | 31.8 µs |
| D230 | 2.82 µs | 10.1 µs | 19.8 µs | 23.9 µs | 57.1 µs |
| D307 | 3.4 µs | 11.3 µs | 15.9 µs | 70.7 µs | 94 µs |
| D462 | 1.83 µs | 14.1 µs | 58.1 µs | 111 µs | 214 µs |
| D616 | 3.22 µs | 24.2 µs | 120 µs | 266 µs | 451 µs |
| D924 | 3.46 µs | 49.5 µs | 215 µs | 482 µs | 885 µs |
| D1232 | 3.67 µs | 74.6 µs | 460 µs | 887 µs | 2.32 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.4 88.2,206.2 124.4,185.2 160.5,184.8 196.7,188.0 232.9,183.6 269.1,187.5 305.3,183.4 341.5,196.8 377.6,184.6 413.8,183.0 450.0,181.7 450.0,41.8 413.8,62.7 377.6,77.3 341.5,93.4 305.3,111.3 269.1,122.2 232.9,134.9 196.7,146.9 160.5,168.2 124.4,167.9 88.2,185.7 52.0,190.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.4 88.2,206.2 124.4,185.2 160.5,184.8 196.7,188.0 232.9,183.6 269.1,187.5 305.3,183.4 341.5,196.8 377.6,184.6 413.8,183.0 450.0,181.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.6 88.2,191.0 124.4,184.5 160.5,180.6 196.7,177.5 232.9,178.2 269.1,159.8 305.3,157.4 341.5,152.5 377.6,140.8 413.8,125.3 450.0,116.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,185.6 124.4,175.9 160.5,174.6 196.7,159.8 232.9,167.3 269.1,145.2 305.3,149.9 341.5,121.8 377.6,106.0 413.8,93.4 450.0,76.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.6 88.2,183.6 124.4,171.3 160.5,167.3 196.7,152.9 232.9,151.5 269.1,141.1 305.3,117.5 341.5,107.7 377.6,88.8 413.8,75.9 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.7 88.2,185.7 124.4,167.9 160.5,168.2 196.7,146.9 232.9,134.9 269.1,122.2 305.3,111.3 341.5,93.4 377.6,77.3 413.8,62.7 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.71 µs | 1.74 µs | 3.63 µs | 3.57 µs | 3.28 µs |
| D38 | 1.37 µs | 3.36 µs | 4.35 µs | 4.48 µs | 4.17 µs |
| D57 | 5.66 µs | 5.44 µs | 7.59 µs | 8.48 µs | 8.24 µs |
| D76 | 5.71 µs | 6.4 µs | 7.61 µs | 10.2 µs | 9.93 µs |
| D115 | 11.2 µs | 12.5 µs | 11.8 µs | 22.2 µs | 26.8 µs |
| D153 | 6.36 µs | 6.59 µs | 12 µs | 20.1 µs | 39 µs |
| D230 | 5.2 µs | 14.1 µs | 24.7 µs | 29.4 µs | 69.3 µs |
| D307 | 6.55 µs | 14.6 µs | 33.4 µs | 85.5 µs | 103 µs |
| D462 | 6.13 µs | 21 µs | 82.1 µs | 133 µs | 234 µs |
| D616 | 6.28 µs | 29.2 µs | 135 µs | 289 µs | 448 µs |
| D924 | 6.54 µs | 51.5 µs | 232 µs | 479 µs | 779 µs |
| D1232 | 7.23 µs | 86.9 µs | 450 µs | 784 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.3 88.2,203.2 124.4,172.3 160.5,172.2 196.7,157.6 232.9,169.8 269.1,174.2 305.3,169.2 341.5,170.6 377.6,170.1 413.8,169.2 450.0,167.1 450.0,37.3 413.8,65.4 377.6,77.5 341.5,91.6 305.3,109.4 269.1,118.0 232.9,130.5 196.7,138.6 160.5,160.2 124.4,164.2 88.2,179.0 52.0,184.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.3 88.2,203.2 124.4,172.3 160.5,172.2 196.7,157.6 232.9,169.8 269.1,174.2 305.3,169.2 341.5,170.6 377.6,170.1 413.8,169.2 450.0,167.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.0 88.2,183.7 124.4,173.2 160.5,169.7 196.7,155.1 232.9,169.1 269.1,152.5 305.3,151.7 341.5,143.9 377.6,136.7 413.8,124.4 450.0,113.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,178.1 124.4,166.0 160.5,165.9 196.7,156.4 232.9,156.0 269.1,140.4 305.3,133.8 341.5,114.3 377.6,103.5 413.8,91.7 450.0,77.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.4 88.2,177.4 124.4,163.6 160.5,159.5 196.7,142.6 232.9,144.9 269.1,136.6 305.3,113.4 341.5,103.8 377.6,87.0 413.8,76.0 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.2 88.2,179.0 124.4,164.2 160.5,160.2 196.7,138.6 232.9,130.5 269.1,118.0 305.3,109.4 341.5,91.6 377.6,77.5 413.8,65.4 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.66 µs | 2.14 µs | 4.26 µs | 4.35 µs | 4.03 µs |
| D38 | 2.22 µs | 4.03 µs | 5.13 µs | 5.6 µs | 5.5 µs |
| D57 | 3.96 µs | 4.25 µs | 6.48 µs | 7.79 µs | 6.97 µs |
| D76 | 4 µs | 5.06 µs | 6.88 µs | 9.15 µs | 8.61 µs |
| D115 | 3.55 µs | 6.05 µs | 12.5 µs | 16.9 µs | 22.1 µs |
| D153 | 4.33 µs | 5.78 µs | 9.07 µs | 18.2 µs | 37 µs |
| D230 | 3.59 µs | 12.6 µs | 23.4 µs | 28.5 µs | 64.1 µs |
| D307 | 4.37 µs | 14 µs | 19.2 µs | 80.3 µs | 104 µs |
| D462 | 2.4 µs | 16.7 µs | 66 µs | 124 µs | 233 µs |
| D616 | 4.07 µs | 27.4 µs | 136 µs | 294 µs | 486 µs |
| D924 | 4.33 µs | 49.3 µs | 235 µs | 520 µs | 940 µs |
| D1232 | 4.61 µs | 83.3 µs | 497 µs | 959 µs | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.8 88.2,192.7 124.4,180.1 160.5,179.9 196.7,182.5 232.9,178.2 269.1,182.2 305.3,178.0 341.5,191.0 377.6,179.5 413.8,178.2 450.0,176.8 450.0,40.6 413.8,61.3 377.6,75.7 341.5,91.7 305.3,109.1 269.1,119.7 232.9,131.6 196.7,142.8 160.5,163.2 124.4,167.8 88.2,173.0 52.0,179.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.8 88.2,192.7 124.4,180.1 160.5,179.9 196.7,182.5 232.9,178.2 269.1,182.2 305.3,178.0 341.5,191.0 377.6,179.5 413.8,178.2 450.0,176.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,193.5 88.2,179.7 124.4,178.6 160.5,174.8 196.7,170.9 232.9,171.9 269.1,154.9 305.3,152.6 341.5,148.8 377.6,138.1 413.8,125.4 450.0,114.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.5 88.2,174.5 124.4,169.4 160.5,168.1 196.7,155.2 232.9,162.1 269.1,141.6 305.3,145.9 341.5,119.0 377.6,103.3 413.8,91.4 450.0,75.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.1 88.2,172.6 124.4,165.4 160.5,161.9 196.7,148.6 232.9,147.0 269.1,137.3 305.3,114.8 341.5,105.4 377.6,86.6 413.8,74.2 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,173.0 124.4,167.8 160.5,163.2 196.7,142.8 232.9,131.6 269.1,119.7 305.3,109.1 341.5,91.7 377.6,75.7 413.8,61.3 450.0,40.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.82 µs | 1.76 µs | 3.8 µs | 3.72 µs | 3.45 µs |
| D38 | 1.46 µs | 3.51 µs | 4.42 µs | 4.49 µs | 4.18 µs |
| D57 | 5.82 µs | 5.64 µs | 7.93 µs | 8.83 µs | 8.56 µs |
| D76 | 5.84 µs | 6.5 µs | 7.95 µs | 10.7 µs | 10.2 µs |
| D115 | 11.5 µs | 12.8 µs | 12.4 µs | 22.8 µs | 27.5 µs |
| D153 | 6.48 µs | 6.85 µs | 12.7 µs | 20.7 µs | 41 µs |
| D230 | 5.31 µs | 14.8 µs | 25.5 µs | 29.8 µs | 70.4 µs |
| D307 | 6.73 µs | 15 µs | 37.9 µs | 88.1 µs | 104 µs |
| D462 | 6.3 µs | 21.6 µs | 83.4 µs | 135 µs | 235 µs |
| D616 | 6.53 µs | 29.5 µs | 136 µs | 293 µs | 455 µs |
| D924 | 6.94 µs | 52.6 µs | 235 µs | 482 µs | 785 µs |
| D1232 | 7.57 µs | 87.9 µs | 461 µs | 786 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.0 88.2,201.8 124.4,171.7 160.5,171.7 196.7,157.0 232.9,169.4 269.1,173.7 305.3,168.6 341.5,170.0 377.6,169.3 413.8,167.9 450.0,166.1 450.0,37.2 413.8,65.3 377.6,77.1 341.5,91.4 305.3,109.1 269.1,117.6 232.9,129.4 196.7,138.0 160.5,159.5 124.4,163.4 88.2,178.9 52.0,183.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.0 88.2,201.8 124.4,171.7 160.5,171.7 196.7,157.0 232.9,169.4 269.1,173.7 305.3,168.6 341.5,170.0 377.6,169.3 413.8,167.9 450.0,166.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.7 88.2,182.7 124.4,172.4 160.5,169.3 196.7,154.6 232.9,168.2 269.1,151.6 305.3,151.2 341.5,143.3 377.6,136.5 413.8,123.9 450.0,112.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.0 88.2,177.8 124.4,165.0 160.5,165.0 196.7,155.3 232.9,154.9 269.1,139.7 305.3,131.1 341.5,113.9 377.6,103.2 413.8,91.4 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.4 88.2,177.4 124.4,162.7 160.5,158.6 196.7,142.1 232.9,144.2 269.1,136.3 305.3,112.8 341.5,103.5 377.6,86.7 413.8,75.9 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.1 88.2,178.9 124.4,163.4 160.5,159.5 196.7,138.0 232.9,129.4 269.1,117.6 305.3,109.1 341.5,91.4 377.6,77.1 413.8,65.3 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 147 ns | 138 ns | 173 ns | 160 ns | 137 ns |
| D38 | 115 ns | 161 ns | 179 ns | 186 ns | 185 ns |
| D57 | 241 ns | 302 ns | 350 ns | 355 ns | 325 ns |
| D76 | 238 ns | 328 ns | 333 ns | 414 ns | 355 ns |
| D115 | 369 ns | 564 ns | 723 ns | 791 ns | 901 ns |
| D153 | 546 ns | 554 ns | 501 ns | 702 ns | 1.12 µs |
| D230 | 561 ns | 844 ns | 1.12 µs | 892 ns | 1.58 µs |
| D307 | 1.01 µs | 1.08 µs | 1.01 µs | 2.27 µs | 2.27 µs |
| D462 | 1.06 µs | 1.3 µs | 2.2 µs | 2.65 µs | 3.94 µs |
| D616 | 1.24 µs | 1.42 µs | 2.95 µs | 4.48 µs | 6.1 µs |
| D924 | 1.7 µs | 1.77 µs | 3.89 µs | 6.06 µs | 8.9 µs |
| D1232 | 2.67 µs | 2.64 µs | 7.6 µs | 9.48 µs | 31.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,205.9 124.4,184.5 160.5,184.9 196.7,172.2 232.9,160.8 269.1,160.1 305.3,143.0 341.5,141.7 377.6,137.1 413.8,127.9 450.0,114.9 450.0,43.7 413.8,80.0 377.6,91.0 341.5,103.6 305.3,119.7 269.1,130.1 232.9,140.0 196.7,146.4 160.5,173.3 124.4,175.9 88.2,192.2 52.0,200.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,205.9 124.4,184.5 160.5,184.9 196.7,172.2 232.9,160.8 269.1,160.1 305.3,143.0 341.5,141.7 377.6,137.1 413.8,127.9 450.0,114.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.8 88.2,196.2 124.4,178.0 160.5,175.6 196.7,159.9 232.9,160.4 269.1,148.3 305.3,141.1 341.5,135.8 377.6,133.1 413.8,126.7 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.1 88.2,193.1 124.4,173.8 160.5,175.1 196.7,152.7 232.9,163.3 269.1,140.0 305.3,143.1 341.5,120.5 377.6,112.0 413.8,104.0 450.0,84.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.4 88.2,192.0 124.4,173.3 160.5,168.9 196.7,150.1 232.9,153.6 269.1,146.6 305.3,119.7 341.5,115.2 377.6,99.9 413.8,91.2 450.0,78.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.9 88.2,192.2 124.4,175.9 160.5,173.3 196.7,146.4 232.9,140.0 269.1,130.1 305.3,119.7 341.5,103.6 377.6,91.0 413.8,80.0 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 168 ns | 153 ns | 210 ns | 202 ns | 180 ns |
| D38 | 141 ns | 192 ns | 216 ns | 200 ns | 195 ns |
| D57 | 330 ns | 376 ns | 420 ns | 436 ns | 373 ns |
| D76 | 327 ns | 407 ns | 429 ns | 495 ns | 412 ns |
| D115 | 469 ns | 688 ns | 841 ns | 944 ns | 1 µs |
| D153 | 658 ns | 624 ns | 567 ns | 762 ns | 1.28 µs |
| D230 | 772 ns | 996 ns | 1.26 µs | 1 µs | 1.74 µs |
| D307 | 1.36 µs | 1.28 µs | 1.2 µs | 2.6 µs | 2.61 µs |
| D462 | 1.35 µs | 1.44 µs | 2.56 µs | 2.94 µs | 4.31 µs |
| D616 | 1.63 µs | 1.61 µs | 3.37 µs | 4.91 µs | 6.58 µs |
| D924 | 2.22 µs | 2.05 µs | 4.33 µs | 6.51 µs | 9.48 µs |
| D1232 | 3.41 µs | 3.11 µs | 8.42 µs | 10.3 µs | 32.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,200.1 124.4,175.4 160.5,175.7 196.7,165.2 232.9,155.5 269.1,150.8 305.3,134.4 341.5,134.7 377.6,129.2 413.8,120.3 450.0,107.8 450.0,42.8 413.8,78.2 377.6,88.8 341.5,101.1 305.3,115.5 269.1,127.3 232.9,136.3 196.7,143.2 160.5,169.0 124.4,171.9 88.2,190.7 52.0,192.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,200.1 124.4,175.4 160.5,175.7 196.7,165.2 232.9,155.5 269.1,150.8 305.3,134.4 341.5,134.7 377.6,129.2 413.8,120.3 450.0,107.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.7 88.2,191.1 124.4,171.7 160.5,169.3 196.7,154.2 232.9,157.0 269.1,143.4 305.3,136.2 341.5,132.7 377.6,129.5 413.8,122.5 450.0,110.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.5 88.2,187.7 124.4,168.5 160.5,167.8 196.7,148.4 232.9,159.8 269.1,136.6 305.3,138.0 341.5,116.1 377.6,108.2 413.8,100.9 450.0,81.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.7 88.2,190.0 124.4,167.3 160.5,163.7 196.7,145.0 232.9,151.2 269.1,143.3 305.3,115.7 341.5,112.1 377.6,97.3 413.8,89.1 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.9 88.2,190.7 124.4,171.9 160.5,169.0 196.7,143.2 232.9,136.3 269.1,127.3 305.3,115.5 341.5,101.1 377.6,88.8 413.8,78.2 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
