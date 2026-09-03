# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.14 ns | 2.12 µs | 3.49 µs | 3.05 µs | 4.09 µs |
| D38 | 1.3 µs | 3.97 µs | 4.44 µs | 8.68 µs | 9.1 µs |
| D57 | 1.54 µs | 5.76 µs | 8.77 µs | 11.6 µs | 14.2 µs |
| D76 | 1.46 µs | 4.14 µs | 10.5 µs | 14.4 µs | 18.5 µs |
| D115 | 1.45 µs | 6.47 µs | 20.2 µs | 32.9 µs | 40.3 µs |
| D153 | 1.5 µs | 8.04 µs | 26 µs | 44 µs | 61.8 µs |
| D230 | 1.01 µs | 21 µs | 42.2 µs | 61.2 µs | 133 µs |
| D307 | 899 ns | 25.6 µs | 60.1 µs | 96.5 µs | 192 µs |
| D462 | 1.55 µs | 43.1 µs | 133 µs | 223 µs | 404 µs |
| D616 | 1.17 µs | 41.1 µs | 222 µs | 427 µs | 710 µs |
| D924 | 1.77 µs | 125 µs | 358 µs | 869 µs | 1.67 ms |
| D1232 | 2.07 µs | 219 µs | 712 µs | 1.29 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.0 88.2,121.0 124.4,118.9 160.5,119.6 196.7,119.6 232.9,119.2 269.1,124.1 305.3,125.6 341.5,118.9 377.6,122.4 413.8,117.2 450.0,115.3 450.0,23.0 413.8,32.2 377.6,42.8 341.5,49.8 305.3,59.1 269.1,63.6 232.9,73.1 196.7,78.4 160.5,88.0 124.4,91.3 88.2,96.9 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.0 88.2,121.0 124.4,118.9 160.5,119.6 196.7,119.6 232.9,119.2 269.1,124.1 305.3,125.6 341.5,118.9 377.6,122.4 413.8,117.2 450.0,115.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,107.2 124.4,102.5 160.5,106.6 196.7,101.1 232.9,98.4 269.1,86.5 305.3,84.0 341.5,77.6 377.6,78.2 413.8,64.3 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,105.8 124.4,97.3 160.5,95.1 196.7,87.0 232.9,83.9 269.1,77.9 305.3,73.5 341.5,63.6 377.6,57.2 413.8,51.3 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,97.5 124.4,93.9 160.5,91.2 196.7,80.9 232.9,77.3 269.1,73.2 305.3,67.6 341.5,57.2 377.6,49.1 413.8,40.3 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,96.9 124.4,91.3 160.5,88.0 196.7,78.4 232.9,73.1 269.1,63.6 305.3,59.1 341.5,49.8 377.6,42.8 413.8,32.2 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.23 µs | 3.77 µs | 6.07 µs | 5.38 µs | 7.13 µs |
| D38 | 3.21 µs | 4.87 µs | 7.42 µs | 8.76 µs | 9.31 µs |
| D57 | 3.77 µs | 4.27 µs | 5.43 µs | 6.99 µs | 7.58 µs |
| D76 | 3.39 µs | 2.63 µs | 6.21 µs | 7.77 µs | 9.35 µs |
| D115 | 6.22 µs | 7.02 µs | 12.2 µs | 16.3 µs | 19.6 µs |
| D153 | 6.12 µs | 7.86 µs | 15.7 µs | 21.6 µs | 30.3 µs |
| D230 | 5.28 µs | 16.7 µs | 26.3 µs | 36.9 µs | 73.4 µs |
| D307 | 7.89 µs | 27.8 µs | 48.4 µs | 63.3 µs | 139 µs |
| D462 | 12.9 µs | 37.5 µs | 91.2 µs | 152 µs | 280 µs |
| D616 | 16.1 µs | 45.3 µs | 180 µs | 316 µs | 555 µs |
| D924 | 33.9 µs | 156 µs | 317 µs | 754 µs | 1.43 ms |
| D1232 | 43.2 µs | 279 µs | 735 µs | 1.23 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,184.6 124.4,181.2 160.5,183.5 196.7,170.3 232.9,170.7 269.1,173.9 305.3,165.2 341.5,154.5 377.6,149.6 413.8,133.5 450.0,128.2 450.0,35.5 413.8,52.2 377.6,72.8 341.5,87.7 305.3,102.9 269.1,116.7 232.9,136.0 196.7,145.4 160.5,161.4 124.4,166.0 88.2,161.6 52.0,167.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,184.6 124.4,181.2 160.5,183.5 196.7,170.3 232.9,170.7 269.1,173.9 305.3,165.2 341.5,154.5 377.6,149.6 413.8,133.5 450.0,128.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,175.6 124.4,178.5 160.5,189.0 196.7,167.7 232.9,165.2 269.1,148.8 305.3,137.8 341.5,131.3 377.6,127.2 413.8,100.4 450.0,87.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.8 88.2,166.5 124.4,173.2 160.5,170.4 196.7,155.6 232.9,150.2 269.1,139.0 305.3,125.8 341.5,112.0 377.6,97.2 413.8,85.0 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.4 88.2,162.9 124.4,167.8 160.5,165.5 196.7,149.4 232.9,143.3 269.1,131.6 305.3,119.9 341.5,101.0 377.6,85.0 413.8,66.1 450.0,55.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.3 88.2,161.6 124.4,166.0 160.5,161.4 196.7,145.4 232.9,136.0 269.1,116.7 305.3,102.9 341.5,87.7 377.6,72.8 413.8,52.2 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.1 µs | 3.47 µs | 3.05 µs | 4.06 µs |
| D38 | 1.18 µs | 3.97 µs | 4.36 µs | 8.64 µs | 9.06 µs |
| D57 | 1.43 µs | 5.74 µs | 8.73 µs | 11.5 µs | 14.2 µs |
| D76 | 1.32 µs | 4.13 µs | 10.4 µs | 14.4 µs | 18.4 µs |
| D115 | 1.33 µs | 6.44 µs | 21.1 µs | 32.7 µs | 40.3 µs |
| D153 | 1.38 µs | 8 µs | 25.8 µs | 43.2 µs | 61.8 µs |
| D230 | 941 ns | 20.8 µs | 41.9 µs | 61.6 µs | 133 µs |
| D307 | 819 ns | 25.5 µs | 60 µs | 97.3 µs | 192 µs |
| D462 | 1.4 µs | 43 µs | 133 µs | 222 µs | 403 µs |
| D616 | 1.12 µs | 40.9 µs | 224 µs | 426 µs | 711 µs |
| D924 | 1.68 µs | 126 µs | 358 µs | 868 µs | 1.67 ms |
| D1232 | 1.95 µs | 218 µs | 714 µs | 1.3 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,122.2 124.4,119.9 160.5,120.8 196.7,120.8 232.9,120.3 269.1,125.0 305.3,126.8 341.5,120.1 377.6,122.9 413.8,117.8 450.0,116.0 450.0,23.0 413.8,32.2 377.6,42.8 341.5,49.8 305.3,59.1 269.1,63.6 232.9,73.1 196.7,78.4 160.5,88.1 124.4,91.3 88.2,96.9 52.0,106.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,122.2 124.4,119.9 160.5,120.8 196.7,120.8 232.9,120.3 269.1,125.0 305.3,126.8 341.5,120.1 377.6,122.9 413.8,117.8 450.0,116.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,107.2 124.4,102.6 160.5,106.7 196.7,101.2 232.9,98.5 269.1,86.6 305.3,84.1 341.5,77.6 377.6,78.2 413.8,64.3 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.0 124.4,97.4 160.5,95.2 196.7,86.5 232.9,83.9 269.1,77.9 305.3,73.5 341.5,63.6 377.6,57.2 413.8,51.3 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,97.5 124.4,94.0 160.5,91.2 196.7,81.0 232.9,77.6 269.1,73.2 305.3,67.5 341.5,57.2 377.6,49.1 413.8,40.3 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,96.9 124.4,91.3 160.5,88.1 196.7,78.4 232.9,73.1 269.1,63.6 305.3,59.1 341.5,49.8 377.6,42.8 413.8,32.2 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 2.66 µs | 2.37 µs | 3.12 µs |
| D38 | 4.2 ns | 2.12 µs | 3.26 µs | 3.92 µs | 4.15 µs |
| D57 | 2.11 ns | 5.36 µs | 7.92 µs | 10.8 µs | 11.9 µs |
| D76 | 2.34 ns | 3.71 µs | 9.25 µs | 11.8 µs | 14 µs |
| D115 | 14 ns | 10.7 µs | 19.1 µs | 24.8 µs | 31.4 µs |
| D153 | 16.5 ns | 12.5 µs | 23.5 µs | 34.3 µs | 43.8 µs |
| D230 | 15.4 ns | 26.1 µs | 41.1 µs | 54 µs | 102 µs |
| D307 | 31 ns | 39.8 µs | 67.1 µs | 87.2 µs | 177 µs |
| D462 | 69.5 ns | 56.2 µs | 126 µs | 178 µs | 338 µs |
| D616 | 62.3 ns | 65.5 µs | 246 µs | 401 µs | 668 µs |
| D924 | 118 ns | 231 µs | 421 µs | 871 µs | 1.6 ms |
| D1232 | 148 ns | 396 µs | 939 µs | 1.41 ms | 3.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.2 124.4,200.7 160.5,199.4 196.7,177.2 232.9,175.2 269.1,176.1 305.3,167.4 341.5,157.4 377.6,158.7 413.8,150.8 450.0,148.0 450.0,24.4 413.8,32.8 377.6,43.6 341.5,52.0 305.3,60.1 269.1,66.9 232.9,77.4 196.7,81.5 160.5,91.6 124.4,93.6 88.2,106.6 52.0,110.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.2 124.4,200.7 160.5,199.4 196.7,177.2 232.9,175.2 269.1,176.1 305.3,167.4 341.5,157.4 377.6,158.7 413.8,150.8 450.0,148.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,114.9 124.4,103.4 160.5,108.0 196.7,94.9 232.9,93.0 269.1,83.8 305.3,78.6 341.5,74.3 377.6,72.4 413.8,56.8 450.0,50.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,109.6 124.4,98.6 160.5,96.7 196.7,87.7 232.9,85.1 269.1,78.2 305.3,72.1 341.5,64.3 377.6,56.0 413.8,49.3 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.6 88.2,107.3 124.4,94.8 160.5,93.7 196.7,84.4 232.9,80.4 269.1,74.8 305.3,68.8 341.5,60.0 377.6,49.9 413.8,40.3 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,106.6 124.4,93.6 160.5,91.6 196.7,81.5 232.9,77.4 269.1,66.9 305.3,60.1 341.5,52.0 377.6,43.6 413.8,32.8 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.64 µs | 2.92 µs | 2.6 µs | 3.49 µs |
| D38 | 632 ns | 3.41 µs | 3.14 µs | 6.78 µs | 7.21 µs |
| D57 | 577 ns | 4.45 µs | 6.74 µs | 8.92 µs | 5.3 µs |
| D76 | 524 ns | 3.32 µs | 8.25 µs | 11.9 µs | 15.5 µs |
| D115 | 529 ns | 4.87 µs | 16.7 µs | 29.5 µs | 36.4 µs |
| D153 | 573 ns | 6.26 µs | 17 µs | 39 µs | 55.6 µs |
| D230 | 416 ns | 18.1 µs | 38.5 µs | 56.5 µs | 122 µs |
| D307 | 337 ns | 22 µs | 49.4 µs | 88.7 µs | 178 µs |
| D462 | 602 ns | 34.7 µs | 115 µs | 195 µs | 345 µs |
| D616 | 489 ns | 37.9 µs | 209 µs | 400 µs | 673 µs |
| D924 | 805 ns | 115 µs | 336 µs | 830 µs | 1.57 ms |
| D1232 | 1.01 µs | 204 µs | 676 µs | 1.25 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,130.0 124.4,131.1 160.5,132.3 196.7,132.2 232.9,131.2 269.1,135.2 305.3,137.8 341.5,130.6 377.6,133.2 413.8,127.0 450.0,124.2 450.0,23.6 413.8,32.9 377.6,43.5 341.5,51.8 305.3,60.0 269.1,64.7 232.9,74.4 196.7,79.7 160.5,90.3 124.4,103.6 88.2,99.8 52.0,108.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,130.0 124.4,131.1 160.5,132.3 196.7,132.2 232.9,131.2 269.1,135.2 305.3,137.8 341.5,130.6 377.6,133.2 413.8,127.0 450.0,124.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.1 88.2,109.1 124.4,105.8 160.5,109.4 196.7,104.6 232.9,101.5 269.1,88.3 305.3,85.9 341.5,80.3 377.6,79.2 413.8,65.4 450.0,58.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,110.1 124.4,100.6 160.5,98.1 196.7,89.4 232.9,89.1 269.1,79.0 305.3,75.9 341.5,65.4 377.6,58.0 413.8,52.1 450.0,43.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,100.5 124.4,97.1 160.5,93.6 196.7,82.3 232.9,78.8 269.1,74.2 305.3,68.6 341.5,58.9 377.6,49.9 413.8,40.9 450.0,35.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,99.8 124.4,103.6 160.5,90.3 196.7,79.7 232.9,74.4 269.1,64.7 305.3,60.0 341.5,51.8 377.6,43.5 413.8,32.9 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 3 µs | 2.66 µs | 3.61 µs |
| D38 | 4.17 ns | 2.39 µs | 3.75 µs | 4.36 µs | 4.68 µs |
| D57 | 522 ns | 5.64 µs | 7.6 µs | 9.52 µs | 11.2 µs |
| D76 | 445 ns | 3.53 µs | 8.75 µs | 11.3 µs | 14.2 µs |
| D115 | 896 ns | 9.46 µs | 17.6 µs | 26 µs | 30.3 µs |
| D153 | 912 ns | 11.2 µs | 23.8 µs | 33.8 µs | 48.2 µs |
| D230 | 783 ns | 24.5 µs | 40.7 µs | 61.9 µs | 124 µs |
| D307 | 1.28 µs | 42.4 µs | 80.3 µs | 109 µs | 245 µs |
| D462 | 1.95 µs | 58.3 µs | 156 µs | 272 µs | 508 µs |
| D616 | 2.43 µs | 72.8 µs | 313 µs | 567 µs | 1.02 ms |
| D924 | 5.2 µs | 259 µs | 566 µs | 1.39 ms | 2.66 ms |
| D1232 | 6.69 µs | 480 µs | 1.32 ms | 2.28 ms | 5.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.3 124.4,132.3 160.5,134.3 196.7,125.6 232.9,125.4 269.1,127.3 305.3,121.2 341.5,116.0 377.6,113.3 413.8,103.8 450.0,100.7 450.0,16.7 413.8,26.4 377.6,38.4 341.5,47.0 305.3,56.0 269.1,64.4 232.9,76.2 196.7,82.0 160.5,91.4 124.4,94.3 88.2,105.1 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.3 124.4,132.3 160.5,134.3 196.7,125.6 232.9,125.4 269.1,127.3 305.3,121.2 341.5,116.0 377.6,113.3 413.8,103.8 450.0,100.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.8 88.2,113.4 124.4,102.8 160.5,108.6 196.7,96.4 232.9,94.3 269.1,84.6 305.3,77.8 341.5,73.8 377.6,71.1 413.8,55.3 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,107.9 124.4,99.1 160.5,97.4 196.7,88.7 232.9,84.9 269.1,78.3 305.3,69.9 341.5,61.6 377.6,53.0 413.8,45.6 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,106.0 124.4,96.3 160.5,94.2 196.7,83.8 232.9,80.6 269.1,73.1 305.3,66.1 341.5,54.7 377.6,45.6 413.8,34.5 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,105.1 124.4,94.3 160.5,91.4 196.7,82.0 232.9,76.2 269.1,64.4 305.3,56.0 341.5,47.0 377.6,38.4 413.8,26.4 450.0,16.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.98 ns | 1.25 µs | 2.7 µs | 2.29 µs | 3.06 µs |
| D38 | 4.99 ns | 2.17 µs | 3.23 µs | 3.74 µs | 3.07 µs |
| D57 | 2.81 ns | 3.47 µs | 5.09 µs | 6.2 µs | 9.02 µs |
| D76 | 3.12 ns | 2.43 µs | 5.85 µs | 7.27 µs | 9.03 µs |
| D115 | 11.6 ns | 3.57 µs | 9.73 µs | 14.1 µs | 16.6 µs |
| D153 | 16.8 ns | 4.25 µs | 9.99 µs | 19.4 µs | 29.7 µs |
| D230 | 21.3 ns | 10.3 µs | 18.2 µs | 31.7 µs | 73.2 µs |
| D307 | 42.8 ns | 12.5 µs | 24.4 µs | 53 µs | 115 µs |
| D462 | 110 ns | 15.6 µs | 66.5 µs | 125 µs | 235 µs |
| D616 | 118 ns | 19.4 µs | 130 µs | 261 µs | 463 µs |
| D924 | 173 ns | 70.2 µs | 217 µs | 571 µs | 1.13 ms |
| D1232 | 362 ns | 129 µs | 459 µs | 898 µs | 2.47 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,190.0 124.4,197.2 160.5,195.9 196.7,179.6 232.9,175.0 269.1,172.0 305.3,163.4 341.5,151.7 377.6,150.8 413.8,146.0 450.0,136.9 450.0,27.3 413.8,37.0 377.6,48.1 341.5,56.5 305.3,65.4 269.1,71.0 232.9,82.2 196.7,89.4 160.5,97.0 124.4,97.0 88.2,110.4 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,190.0 124.4,197.2 160.5,195.9 196.7,179.6 232.9,175.0 269.1,172.0 305.3,163.4 341.5,151.7 377.6,150.8 413.8,146.0 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.5 88.2,114.7 124.4,108.9 160.5,113.3 196.7,108.5 232.9,106.3 269.1,95.3 305.3,92.9 341.5,90.2 377.6,87.5 413.8,71.5 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.7 124.4,104.1 160.5,102.4 196.7,96.1 232.9,95.7 269.1,88.3 305.3,84.6 341.5,72.2 377.6,63.9 413.8,57.5 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.0 88.2,107.9 124.4,101.6 160.5,99.7 196.7,91.5 232.9,87.5 269.1,81.4 305.3,75.0 341.5,64.4 377.6,55.2 413.8,45.5 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,110.4 124.4,97.0 160.5,97.0 196.7,89.4 232.9,82.2 269.1,71.0 305.3,65.4 341.5,56.5 377.6,48.1 413.8,37.0 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.74 ns | 1.9 µs | 3.63 µs | 3.08 µs | 4.13 µs |
| D38 | 4.42 ns | 2.88 µs | 4.3 µs | 4.84 µs | 4.15 µs |
| D57 | 3.17 ns | 5.68 µs | 7.64 µs | 8.8 µs | 10.4 µs |
| D76 | 3.43 ns | 3.66 µs | 8.1 µs | 10.3 µs | 12.1 µs |
| D115 | 10.3 ns | 10.1 µs | 12.1 µs | 21.9 µs | 24.5 µs |
| D153 | 16.5 ns | 6.41 µs | 16.4 µs | 24.1 µs | 36.3 µs |
| D230 | 21.2 ns | 14.6 µs | 23.2 µs | 37.7 µs | 85.6 µs |
| D307 | 42.8 ns | 17.2 µs | 50.4 µs | 60.4 µs | 125 µs |
| D462 | 109 ns | 23.8 µs | 87.2 µs | 149 µs | 253 µs |
| D616 | 132 ns | 24.2 µs | 144 µs | 272 µs | 448 µs |
| D924 | 195 ns | 82.7 µs | 230 µs | 562 µs | 999 µs |
| D1232 | 368 ns | 144 µs | 450 µs | 783 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,191.6 124.4,195.7 160.5,194.7 196.7,181.1 232.9,175.2 269.1,172.1 305.3,163.4 341.5,151.7 377.6,149.4 413.8,144.5 450.0,136.7 450.0,25.6 413.8,38.6 377.6,48.5 341.5,55.6 305.3,64.4 269.1,69.1 232.9,79.7 196.7,84.6 160.5,93.4 124.4,95.2 88.2,106.6 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,191.6 124.4,195.7 160.5,194.7 196.7,181.1 232.9,175.2 269.1,172.1 305.3,163.4 341.5,151.7 377.6,149.4 413.8,144.5 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.3 88.2,111.2 124.4,102.7 160.5,108.2 196.7,95.6 232.9,101.2 269.1,91.0 305.3,89.0 341.5,85.0 377.6,84.7 413.8,69.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,106.2 124.4,99.1 160.5,98.3 196.7,93.3 232.9,89.5 269.1,85.3 305.3,75.6 341.5,68.8 377.6,62.6 413.8,56.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,104.7 124.4,97.3 160.5,95.4 196.7,86.0 232.9,84.8 269.1,79.2 305.3,73.4 341.5,62.2 377.6,54.7 413.8,45.7 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,106.6 124.4,95.2 160.5,93.4 196.7,84.6 232.9,79.7 269.1,69.1 305.3,64.4 341.5,55.6 377.6,48.5 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.13 µs | 2.53 µs | 2.24 µs | 3 µs |
| D38 | 4.37 ns | 2.03 µs | 3.06 µs | 3.56 µs | 3.04 µs |
| D57 | 2.81 ns | 3.26 µs | 4.84 µs | 6.1 µs | 9.03 µs |
| D76 | 3.43 ns | 2.23 µs | 5.58 µs | 7.22 µs | 8.79 µs |
| D115 | 11.6 ns | 3.45 µs | 10.2 µs | 14.3 µs | 16.4 µs |
| D153 | 17.1 ns | 4.07 µs | 10.6 µs | 19 µs | 29.4 µs |
| D230 | 21.3 ns | 10.2 µs | 18.6 µs | 31.6 µs | 70 µs |
| D307 | 38.1 ns | 12.3 µs | 23.7 µs | 53.4 µs | 113 µs |
| D462 | 102 ns | 15.5 µs | 63.1 µs | 124 µs | 232 µs |
| D616 | 114 ns | 18.9 µs | 130 µs | 257 µs | 460 µs |
| D924 | 190 ns | 68 µs | 214 µs | 566 µs | 1.12 ms |
| D1232 | 362 ns | 127 µs | 460 µs | 889 µs | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,194.7 196.7,179.6 232.9,174.7 269.1,172.1 305.3,164.8 341.5,152.6 377.6,151.2 413.8,144.9 450.0,136.9 450.0,27.5 413.8,37.1 377.6,48.2 341.5,56.7 305.3,65.6 269.1,71.6 232.9,82.4 196.7,89.5 160.5,97.3 124.4,97.0 88.2,110.5 52.0,110.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,194.7 196.7,179.6 232.9,174.7 269.1,172.1 305.3,164.8 341.5,152.6 377.6,151.2 413.8,144.9 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.8 88.2,115.5 124.4,109.6 160.5,114.3 196.7,108.9 232.9,106.9 269.1,95.5 305.3,93.2 341.5,90.3 377.6,87.8 413.8,71.9 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,110.4 124.4,104.7 160.5,102.9 196.7,95.5 232.9,95.0 269.1,88.0 305.3,85.0 341.5,72.9 377.6,63.9 413.8,57.7 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.3 88.2,108.5 124.4,101.8 160.5,99.8 196.7,91.3 232.9,87.8 269.1,81.4 305.3,74.9 341.5,64.4 377.6,55.4 413.8,45.6 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.6 88.2,110.5 124.4,97.0 160.5,97.3 196.7,89.5 232.9,82.4 269.1,71.6 305.3,65.6 341.5,56.7 377.6,48.2 413.8,37.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.91 µs | 3.63 µs | 3.09 µs | 4.14 µs |
| D38 | 4.39 ns | 2.88 µs | 4.3 µs | 4.84 µs | 4.15 µs |
| D57 | 12.3 ns | 5.72 µs | 7.66 µs | 8.85 µs | 10.5 µs |
| D76 | 10.4 ns | 3.66 µs | 8.12 µs | 10.2 µs | 12.1 µs |
| D115 | 10.6 ns | 10.4 µs | 12.2 µs | 22 µs | 24.5 µs |
| D153 | 15.9 ns | 6.45 µs | 16.2 µs | 23.6 µs | 36.2 µs |
| D230 | 21.2 ns | 14.7 µs | 23 µs | 37.7 µs | 85.9 µs |
| D307 | 37.9 ns | 17.2 µs | 49.4 µs | 62.4 µs | 124 µs |
| D462 | 102 ns | 23.9 µs | 87.5 µs | 149 µs | 254 µs |
| D616 | 108 ns | 23.9 µs | 146 µs | 273 µs | 448 µs |
| D924 | 216 ns | 82.6 µs | 231 µs | 561 µs | 998 µs |
| D1232 | 377 ns | 143 µs | 450 µs | 782 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,191.6 124.4,178.9 160.5,180.9 196.7,180.7 232.9,175.7 269.1,172.1 305.3,164.9 341.5,152.7 377.6,151.9 413.8,143.3 450.0,136.4 450.0,25.6 413.8,38.6 377.6,48.5 341.5,55.6 305.3,64.4 269.1,69.0 232.9,79.8 196.7,84.6 160.5,93.3 124.4,95.1 88.2,106.6 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,191.6 124.4,178.9 160.5,180.9 196.7,180.7 232.9,175.7 269.1,172.1 305.3,164.9 341.5,152.7 377.6,151.9 413.8,143.3 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.3 88.2,111.2 124.4,102.6 160.5,108.2 196.7,95.2 232.9,101.2 269.1,90.9 305.3,89.0 341.5,84.9 377.6,84.9 413.8,69.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,106.2 124.4,99.0 160.5,98.3 196.7,93.3 232.9,89.8 269.1,85.4 305.3,75.9 341.5,68.8 377.6,62.5 413.8,56.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,104.7 124.4,97.2 160.5,95.4 196.7,85.9 232.9,85.0 269.1,79.2 305.3,73.0 341.5,62.2 377.6,54.7 413.8,45.7 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,106.6 124.4,95.1 160.5,93.3 196.7,84.6 232.9,79.8 269.1,69.0 305.3,64.4 341.5,55.6 377.6,48.5 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.29 µs | 4.28 µs | 3.72 µs | 4.92 µs |
| D38 | 4.69 ns | 3.48 µs | 5.13 µs | 5.92 µs | 5.39 µs |
| D57 | 2.81 ns | 4.35 µs | 6.42 µs | 8.2 µs | 9.35 µs |
| D76 | 3.74 ns | 2.91 µs | 7.43 µs | 9.38 µs | 11.1 µs |
| D115 | 10.9 ns | 4.54 µs | 12.5 µs | 16.8 µs | 20.6 µs |
| D153 | 16.4 ns | 5.44 µs | 12.5 µs | 22.8 µs | 33.8 µs |
| D230 | 20.7 ns | 12.9 µs | 22.2 µs | 36.8 µs | 79.5 µs |
| D307 | 37.5 ns | 15.4 µs | 28.1 µs | 58.3 µs | 127 µs |
| D462 | 96.2 ns | 19.1 µs | 72 µs | 137 µs | 253 µs |
| D616 | 106 ns | 22.2 µs | 141 µs | 281 µs | 491 µs |
| D924 | 159 ns | 77.3 µs | 234 µs | 611 µs | 1.2 ms |
| D1232 | 369 ns | 144 µs | 495 µs | 950 µs | 2.58 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.8 124.4,197.2 160.5,193.6 196.7,180.4 232.9,175.3 269.1,172.4 305.3,165.0 341.5,153.3 377.6,152.2 413.8,147.1 450.0,136.7 450.0,26.8 413.8,36.3 377.6,47.4 341.5,55.6 305.3,64.2 269.1,70.0 232.9,80.6 196.7,86.7 160.5,94.4 124.4,96.6 88.2,103.4 52.0,104.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.8 124.4,197.2 160.5,193.6 196.7,180.4 232.9,175.3 269.1,172.4 305.3,165.0 341.5,153.3 377.6,152.2 413.8,147.1 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.0 88.2,108.8 124.4,106.0 160.5,111.0 196.7,105.5 232.9,103.3 269.1,92.6 305.3,90.3 341.5,87.7 377.6,85.8 413.8,70.3 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,104.0 124.4,101.2 160.5,99.4 196.7,92.9 232.9,92.9 269.1,85.8 305.3,82.9 341.5,71.2 377.6,62.8 413.8,56.6 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,102.2 124.4,98.2 160.5,96.5 196.7,89.3 232.9,85.5 269.1,79.6 305.3,73.8 341.5,63.3 377.6,54.3 413.8,44.7 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.5 88.2,103.4 124.4,96.6 160.5,94.4 196.7,86.7 232.9,80.6 269.1,70.0 305.3,64.2 341.5,55.6 377.6,47.4 413.8,36.3 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.94 µs | 3.82 µs | 3.23 µs | 4.33 µs |
| D38 | 4.16 ns | 3.03 µs | 4.35 µs | 4.92 µs | 4.18 µs |
| D57 | 2.79 µs | 5.76 µs | 8.07 µs | 9.34 µs | 11 µs |
| D76 | 2.56 µs | 3.77 µs | 8.42 µs | 10.8 µs | 12.6 µs |
| D115 | 5.32 µs | 10.6 µs | 12.6 µs | 22.6 µs | 25.1 µs |
| D153 | 2.76 µs | 6.71 µs | 17 µs | 23.9 µs | 37.2 µs |
| D230 | 1.77 µs | 15.4 µs | 23.8 µs | 38.5 µs | 86.2 µs |
| D307 | 1.8 µs | 17.9 µs | 51.9 µs | 62.1 µs | 127 µs |
| D462 | 3.17 µs | 24.7 µs | 89 µs | 151 µs | 255 µs |
| D616 | 2.46 µs | 25.9 µs | 147 µs | 276 µs | 453 µs |
| D924 | 3.8 µs | 84.2 µs | 235 µs | 565 µs | 1.01 ms |
| D1232 | 4.18 µs | 146 µs | 458 µs | 790 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.3 124.4,111.5 160.5,112.6 196.7,103.6 232.9,111.7 269.1,117.2 305.3,117.0 341.5,110.0 377.6,113.1 413.8,107.7 450.0,106.5 450.0,25.6 413.8,38.5 377.6,48.4 341.5,55.5 305.3,64.2 269.1,69.0 232.9,79.4 196.7,84.3 160.5,92.8 124.4,94.6 88.2,106.5 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.3 124.4,111.5 160.5,112.6 196.7,103.6 232.9,111.7 269.1,117.2 305.3,117.0 341.5,110.0 377.6,113.1 413.8,107.7 450.0,106.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.0 88.2,110.5 124.4,102.6 160.5,107.8 196.7,95.0 232.9,100.7 269.1,90.4 305.3,88.5 341.5,84.5 377.6,83.9 413.8,69.3 450.0,62.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,106.0 124.4,98.4 160.5,97.8 196.7,92.8 232.9,89.1 269.1,85.0 305.3,75.3 341.5,68.6 377.6,62.3 413.8,56.6 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,104.5 124.4,96.6 160.5,94.7 196.7,85.6 232.9,84.9 269.1,79.0 305.3,73.1 341.5,62.0 377.6,54.5 413.8,45.6 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,106.5 124.4,94.6 160.5,92.8 196.7,84.3 232.9,79.4 269.1,69.0 305.3,64.2 341.5,55.5 377.6,48.4 413.8,38.5 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 140 ns | 170 ns | 133 ns | 178 ns |
| D38 | 4.38 ns | 132 ns | 176 ns | 199 ns | 184 ns |
| D57 | 197 ns | 307 ns | 348 ns | 347 ns | 413 ns |
| D76 | 179 ns | 170 ns | 353 ns | 420 ns | 432 ns |
| D115 | 404 ns | 440 ns | 708 ns | 799 ns | 760 ns |
| D153 | 410 ns | 450 ns | 820 ns | 878 ns | 1.01 µs |
| D230 | 307 ns | 901 ns | 982 ns | 1.13 µs | 1.95 µs |
| D307 | 500 ns | 1.34 µs | 1.53 µs | 1.55 µs | 2.81 µs |
| D462 | 888 ns | 1.39 µs | 2.31 µs | 2.96 µs | 4.16 µs |
| D616 | 776 ns | 1.18 µs | 3.1 µs | 4.12 µs | 6.05 µs |
| D924 | 1.6 µs | 2.74 µs | 3.87 µs | 6.96 µs | 11.3 µs |
| D1232 | 2.25 µs | 4.26 µs | 7.46 µs | 9.36 µs | 30.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,118.2 160.5,119.9 196.7,105.7 232.9,105.5 269.1,110.5 305.3,102.0 341.5,92.1 377.6,94.4 413.8,81.9 450.0,75.9 450.0,30.8 413.8,47.9 377.6,58.7 341.5,65.2 305.3,72.1 269.1,78.4 232.9,89.9 196.7,94.8 160.5,104.6 124.4,105.4 88.2,119.4 52.0,119.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,118.2 160.5,119.9 196.7,105.7 232.9,105.5 269.1,110.5 305.3,102.0 341.5,92.1 377.6,94.4 413.8,81.9 450.0,75.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.2 88.2,125.2 124.4,110.5 160.5,120.8 196.7,104.3 232.9,103.9 269.1,91.8 305.3,84.9 341.5,84.2 377.6,87.1 413.8,72.5 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.8 88.2,120.2 124.4,108.3 160.5,108.1 196.7,96.0 232.9,93.4 269.1,90.3 305.3,82.6 341.5,75.5 377.6,70.4 413.8,66.5 450.0,55.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,125.0 88.2,118.0 124.4,108.4 160.5,105.1 196.7,93.9 232.9,92.3 269.1,87.8 305.3,82.4 341.5,71.2 377.6,65.4 413.8,56.3 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.9 88.2,119.4 124.4,105.4 160.5,104.6 196.7,94.8 232.9,89.9 269.1,78.4 305.3,72.1 341.5,65.2 377.6,58.7 413.8,47.9 450.0,30.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 168 ns | 210 ns | 178 ns | 215 ns |
| D38 | 4.38 ns | 172 ns | 217 ns | 216 ns | 193 ns |
| D57 | 309 ns | 401 ns | 448 ns | 477 ns | 523 ns |
| D76 | 281 ns | 233 ns | 489 ns | 535 ns | 569 ns |
| D115 | 623 ns | 556 ns | 943 ns | 1.05 µs | 935 ns |
| D153 | 601 ns | 580 ns | 1.02 µs | 1.08 µs | 1.28 µs |
| D230 | 537 ns | 1.26 µs | 1.29 µs | 1.5 µs | 2.3 µs |
| D307 | 822 ns | 1.86 µs | 2.08 µs | 2.02 µs | 3.37 µs |
| D462 | 1.43 µs | 1.87 µs | 2.95 µs | 3.5 µs | 4.85 µs |
| D616 | 1.25 µs | 1.48 µs | 3.89 µs | 4.88 µs | 6.93 µs |
| D924 | 2.57 µs | 3.58 µs | 4.61 µs | 7.88 µs | 12.4 µs |
| D1232 | 3.49 µs | 5.51 µs | 8.71 µs | 10.6 µs | 31.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,110.4 160.5,112.1 196.7,98.2 232.9,98.8 269.1,100.8 305.3,93.4 341.5,83.8 377.6,86.1 413.8,73.6 450.0,68.3 450.0,30.0 413.8,46.3 377.6,56.4 341.5,62.6 305.3,68.9 269.1,75.5 232.9,85.7 196.7,91.2 160.5,99.8 124.4,101.3 88.2,118.6 52.0,116.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,110.4 160.5,112.1 196.7,98.2 232.9,98.8 269.1,100.8 305.3,93.4 341.5,83.8 377.6,86.1 413.8,73.6 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.9 88.2,120.6 124.4,105.9 160.5,115.3 196.7,100.2 232.9,99.5 269.1,85.9 305.3,79.2 341.5,79.1 377.6,83.2 413.8,67.8 450.0,60.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.1 88.2,116.5 124.4,104.0 160.5,102.4 196.7,91.0 232.9,89.7 269.1,85.6 305.3,77.3 341.5,71.2 377.6,66.4 413.8,63.5 450.0,52.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.0 88.2,116.6 124.4,102.9 160.5,100.9 196.7,89.2 232.9,88.7 269.1,83.0 305.3,77.8 341.5,68.2 377.6,62.5 413.8,54.1 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.7 88.2,118.6 124.4,101.3 160.5,99.8 196.7,91.2 232.9,85.7 269.1,75.5 305.3,68.9 341.5,62.6 377.6,56.4 413.8,46.3 450.0,30.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
