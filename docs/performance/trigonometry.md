# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.79 ns | 1.99 µs | 3.49 µs | 3.49 µs | 4.08 µs |
| D38 | 1.32 µs | 4.67 µs | 4.16 µs | 7.86 µs | 6.68 µs |
| D57 | 1.44 µs | 6.33 µs | 8.74 µs | 10.9 µs | 15 µs |
| D76 | 1.43 µs | 6.45 µs | 9.94 µs | 15.3 µs | 10.9 µs |
| D115 | 1.63 µs | 8.28 µs | 16.5 µs | 32.6 µs | 28.6 µs |
| D153 | 1.46 µs | 10.4 µs | 20.8 µs | 36.2 µs | 47.9 µs |
| D230 | 1.29 µs | 20.2 µs | 46 µs | 61.8 µs | 107 µs |
| D307 | 1.54 µs | 18.3 µs | 66.1 µs | 132 µs | 180 µs |
| D462 | 1.64 µs | 42.6 µs | 132 µs | 239 µs | 313 µs |
| D616 | 1.56 µs | 47.8 µs | 206 µs | 388 µs | 708 µs |
| D924 | 993 ns | 123 µs | 450 µs | 876 µs | 1.67 ms |
| D1232 | 1.11 µs | 218 µs | 663 µs | 1.66 ms | 3.32 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,120.9 124.4,119.8 160.5,119.9 196.7,118.2 232.9,119.6 269.1,121.1 305.3,118.9 341.5,118.1 377.6,118.8 413.8,124.4 450.0,123.0 450.0,23.7 413.8,32.2 377.6,42.9 341.5,53.0 305.3,59.8 269.1,66.3 232.9,76.3 196.7,82.7 160.5,94.6 124.4,90.7 88.2,100.7 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,120.9 124.4,119.8 160.5,119.9 196.7,118.2 232.9,119.6 269.1,121.1 305.3,118.9 341.5,118.1 377.6,118.8 413.8,124.4 450.0,123.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.2 124.4,101.4 160.5,101.2 196.7,98.1 232.9,95.3 269.1,87.0 305.3,88.2 341.5,77.7 377.6,76.3 413.8,64.5 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.6 124.4,97.4 160.5,95.8 196.7,89.5 232.9,86.6 269.1,76.8 305.3,72.3 341.5,63.7 377.6,58.1 413.8,48.5 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.7 124.4,94.7 160.5,90.4 196.7,81.1 232.9,79.8 269.1,73.1 305.3,63.7 341.5,56.3 377.6,50.3 413.8,40.2 450.0,32.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,100.7 124.4,90.7 160.5,94.6 196.7,82.7 232.9,76.3 269.1,66.3 305.3,59.8 341.5,53.0 377.6,42.9 413.8,32.2 450.0,23.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.2 µs | 3.61 µs | 6.06 µs | 6.38 µs | 7.12 µs |
| D38 | 3.25 µs | 5.83 µs | 7.1 µs | 8.39 µs | 7.66 µs |
| D57 | 3.43 µs | 4.81 µs | 5.39 µs | 6.18 µs | 8.2 µs |
| D76 | 3.38 µs | 4.59 µs | 5.82 µs | 8.37 µs | 5.6 µs |
| D115 | 6.12 µs | 8.45 µs | 10.2 µs | 16.5 µs | 13.7 µs |
| D153 | 6.23 µs | 9.86 µs | 12.3 µs | 17.3 µs | 24.1 µs |
| D230 | 7.56 µs | 16.1 µs | 28.7 µs | 37.1 µs | 58.9 µs |
| D307 | 13.6 µs | 17.1 µs | 50.9 µs | 89.8 µs | 133 µs |
| D462 | 13.3 µs | 36 µs | 89.2 µs | 159 µs | 221 µs |
| D616 | 22 µs | 50.4 µs | 169 µs | 289 µs | 559 µs |
| D924 | 20.1 µs | 153 µs | 396 µs | 758 µs | 1.43 ms |
| D1232 | 26.7 µs | 280 µs | 691 µs | 1.56 ms | 2.93 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.7 88.2,184.4 124.4,183.3 160.5,183.5 196.7,170.7 232.9,170.3 269.1,166.1 305.3,153.4 341.5,153.8 377.6,142.9 413.8,144.8 450.0,138.7 450.0,36.6 413.8,52.2 377.6,72.6 341.5,92.8 305.3,103.8 269.1,121.5 232.9,140.9 196.7,153.1 160.5,172.6 124.4,164.3 88.2,165.8 52.0,167.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.7 88.2,184.4 124.4,183.3 160.5,183.5 196.7,170.7 232.9,170.3 269.1,166.1 305.3,153.4 341.5,153.8 377.6,142.9 413.8,144.8 450.0,138.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.1 88.2,171.7 124.4,175.9 160.5,176.9 196.7,163.6 232.9,160.3 269.1,149.7 305.3,148.3 341.5,132.2 377.6,124.9 413.8,100.8 450.0,87.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.9 88.2,167.4 124.4,173.4 160.5,171.8 196.7,159.6 232.9,155.5 269.1,137.1 305.3,124.6 341.5,112.5 377.6,98.6 413.8,80.1 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.8 88.2,163.8 124.4,170.4 160.5,163.9 196.7,149.1 232.9,148.0 269.1,131.5 305.3,112.3 341.5,100.0 377.6,87.0 413.8,66.0 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.4 88.2,165.8 124.4,164.3 160.5,172.6 196.7,153.1 232.9,140.9 269.1,121.5 305.3,103.8 341.5,92.8 377.6,72.6 413.8,52.2 450.0,36.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.98 µs | 3.47 µs | 3.47 µs | 4.05 µs |
| D38 | 1.21 µs | 4.64 µs | 4.13 µs | 7.79 µs | 6.66 µs |
| D57 | 1.31 µs | 6.35 µs | 8.66 µs | 10.8 µs | 15 µs |
| D76 | 1.31 µs | 6.42 µs | 9.94 µs | 15.3 µs | 11 µs |
| D115 | 1.48 µs | 8.25 µs | 17.3 µs | 32.5 µs | 28.1 µs |
| D153 | 1.36 µs | 10.4 µs | 20.7 µs | 38.7 µs | 48.5 µs |
| D230 | 1.19 µs | 19.9 µs | 45.9 µs | 62.1 µs | 107 µs |
| D307 | 1.45 µs | 15.5 µs | 65.8 µs | 132 µs | 181 µs |
| D462 | 1.53 µs | 42.5 µs | 134 µs | 239 µs | 313 µs |
| D616 | 1.44 µs | 47.7 µs | 205 µs | 388 µs | 708 µs |
| D924 | 938 ns | 123 µs | 447 µs | 879 µs | 1.67 ms |
| D1232 | 1.04 µs | 218 µs | 665 µs | 1.65 ms | 3.32 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,121.9 124.4,120.9 160.5,120.9 196.7,119.4 232.9,120.5 269.1,122.1 305.3,119.7 341.5,119.0 377.6,119.7 413.8,125.1 450.0,123.8 450.0,23.7 413.8,32.2 377.6,42.9 341.5,53.0 305.3,59.8 269.1,66.3 232.9,76.1 196.7,82.9 160.5,94.6 124.4,90.7 88.2,100.8 52.0,106.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,121.9 124.4,120.9 160.5,120.9 196.7,119.4 232.9,120.5 269.1,122.1 305.3,119.7 341.5,119.0 377.6,119.7 413.8,125.1 450.0,123.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.3 124.4,101.3 160.5,101.2 196.7,98.1 232.9,95.3 269.1,87.2 305.3,90.3 341.5,77.8 377.6,76.3 413.8,64.5 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.7 124.4,97.5 160.5,95.8 196.7,88.9 232.9,86.7 269.1,76.8 305.3,72.3 341.5,63.5 377.6,58.2 413.8,48.6 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,98.8 124.4,94.8 160.5,90.4 196.7,81.1 232.9,78.9 269.1,73.0 305.3,63.7 341.5,56.3 377.6,50.3 413.8,40.2 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,100.8 124.4,90.7 160.5,94.6 196.7,82.9 232.9,76.1 269.1,66.3 305.3,59.8 341.5,53.0 377.6,42.9 413.8,32.2 450.0,23.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 µs | 1.71 µs | 2.65 µs | 2.8 µs | 3.11 µs |
| D38 | 1.63 µs | 2.51 µs | 3.09 µs | 3.71 µs | 3.34 µs |
| D57 | 4.42 µs | 5.87 µs | 7.88 µs | 8.79 µs | 12.6 µs |
| D76 | 4.49 µs | 6.23 µs | 8.62 µs | 12.7 µs | 8.4 µs |
| D115 | 8.48 µs | 12.5 µs | 15.8 µs | 24.5 µs | 21.7 µs |
| D153 | 8.59 µs | 14.8 µs | 18.6 µs | 28.5 µs | 35.4 µs |
| D230 | 10.4 µs | 24.3 µs | 45.9 µs | 54.2 µs | 82.5 µs |
| D307 | 17.9 µs | 23.9 µs | 72.3 µs | 122 µs | 167 µs |
| D462 | 17.2 µs | 54.5 µs | 126 µs | 195 µs | 259 µs |
| D616 | 29.4 µs | 72.7 µs | 226 µs | 362 µs | 666 µs |
| D924 | 26.3 µs | 227 µs | 522 µs | 870 µs | 1.6 ms |
| D1232 | 35.1 µs | 390 µs | 870 µs | 1.77 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.3 88.2,199.4 124.4,177.7 160.5,177.4 196.7,163.6 232.9,163.3 269.1,159.2 305.3,147.4 341.5,148.2 377.6,136.6 413.8,139.0 450.0,132.8 450.0,37.0 413.8,49.9 377.6,68.8 341.5,89.4 305.3,98.8 269.1,114.2 232.9,132.6 196.7,143.2 160.5,163.8 124.4,155.0 88.2,183.8 52.0,185.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.3 88.2,199.4 124.4,177.7 160.5,177.4 196.7,163.6 232.9,163.3 269.1,159.2 305.3,147.4 341.5,148.2 377.6,136.6 413.8,139.0 450.0,132.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.3 88.2,190.0 124.4,171.6 160.5,170.3 196.7,155.1 232.9,151.5 269.1,140.8 305.3,141.1 341.5,123.2 377.6,116.9 413.8,92.2 450.0,80.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.8 88.2,185.5 124.4,165.2 160.5,163.2 196.7,150.1 232.9,146.5 269.1,126.9 305.3,117.0 341.5,104.9 377.6,92.3 413.8,74.1 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,181.6 124.4,162.8 160.5,154.8 196.7,140.5 232.9,137.3 269.1,123.3 305.3,105.7 341.5,95.5 377.6,82.1 413.8,63.0 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.3 88.2,183.8 124.4,155.0 160.5,163.8 196.7,143.2 232.9,132.6 269.1,114.2 305.3,98.8 341.5,89.4 377.6,68.8 413.8,49.9 450.0,37.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.39 µs | 1.57 µs | 2.91 µs | 2.94 µs | 3.49 µs |
| D38 | 1.82 µs | 2.82 µs | 3.22 µs | 3.55 µs | 2.69 µs |
| D57 | 1.85 µs | 3.16 µs | 3.7 µs | 4.21 µs | 5.64 µs |
| D76 | 5.56 µs | 5.12 µs | 7.78 µs | 12.6 µs | 9.07 µs |
| D115 | 5.94 µs | 6.4 µs | 15.6 µs | 29.4 µs | 26.2 µs |
| D153 | 5.6 µs | 7.98 µs | 13.6 µs | 32.4 µs | 44.5 µs |
| D230 | 4.94 µs | 16.8 µs | 41.8 µs | 56.6 µs | 98.5 µs |
| D307 | 5.95 µs | 13.5 µs | 55.7 µs | 122 µs | 167 µs |
| D462 | 3.06 µs | 34.2 µs | 112 µs | 210 µs | 268 µs |
| D616 | 5.63 µs | 44.7 µs | 190 µs | 362 µs | 669 µs |
| D924 | 3.68 µs | 115 µs | 421 µs | 833 µs | 1.57 ms |
| D1232 | 3.78 µs | 203 µs | 626 µs | 1.59 ms | 3.18 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.0 88.2,197.0 124.4,196.6 160.5,172.7 196.7,171.3 232.9,172.6 269.1,175.3 305.3,171.3 341.5,185.7 377.6,172.5 413.8,181.7 450.0,181.1 450.0,34.9 413.8,50.2 377.6,68.7 341.5,88.6 305.3,98.8 269.1,110.3 232.9,127.6 196.7,139.0 160.5,162.1 124.4,172.4 88.2,188.5 52.0,182.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.0 88.2,197.0 124.4,196.6 160.5,172.7 196.7,171.3 232.9,172.6 269.1,175.3 305.3,171.3 341.5,185.7 377.6,172.5 413.8,181.7 450.0,181.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.2 88.2,187.5 124.4,185.0 160.5,174.5 196.7,169.7 232.9,164.9 269.1,148.8 305.3,153.5 341.5,133.3 377.6,127.5 413.8,106.9 450.0,94.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.8 88.2,184.6 124.4,181.6 160.5,165.4 196.7,150.4 232.9,153.4 269.1,129.0 305.3,122.7 341.5,107.5 377.6,96.0 413.8,78.8 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.6 88.2,182.5 124.4,178.8 160.5,155.0 196.7,136.6 232.9,134.5 269.1,122.3 305.3,105.7 341.5,93.9 377.6,82.0 413.8,64.0 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.9 88.2,188.5 124.4,172.4 160.5,162.1 196.7,139.0 232.9,127.6 269.1,110.3 305.3,98.8 341.5,88.6 377.6,68.7 413.8,50.2 450.0,34.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.83 ns | 1.73 µs | 2.97 µs | 3.15 µs | 3.58 µs |
| D38 | 4.04 ns | 2.86 µs | 3.58 µs | 4.18 µs | 3.84 µs |
| D57 | 442 ns | 6.16 µs | 7.55 µs | 8.93 µs | 12.1 µs |
| D76 | 443 ns | 6.1 µs | 8.17 µs | 12.1 µs | 8.78 µs |
| D115 | 897 ns | 11.7 µs | 14.8 µs | 25.1 µs | 21.1 µs |
| D153 | 899 ns | 13.6 µs | 18.6 µs | 27.1 µs | 37.2 µs |
| D230 | 1.25 µs | 23.1 µs | 43.9 µs | 63.1 µs | 99.1 µs |
| D307 | 2.09 µs | 25.6 µs | 85.9 µs | 152 µs | 236 µs |
| D462 | 2.07 µs | 55.9 µs | 152 µs | 283 µs | 400 µs |
| D616 | 3.52 µs | 82 µs | 292 µs | 524 µs | 1.01 ms |
| D924 | 3.12 µs | 256 µs | 701 µs | 1.39 ms | 2.66 ms |
| D1232 | 4.33 µs | 482 µs | 1.25 ms | 2.9 ms | 5.52 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.3 88.2,192.7 124.4,134.4 160.5,134.4 196.7,125.6 232.9,125.6 269.1,121.5 305.3,115.1 341.5,115.2 377.6,108.7 413.8,110.2 450.0,106.1 450.0,17.4 413.8,26.4 377.6,38.5 341.5,50.0 305.3,56.5 269.1,67.3 232.9,79.4 196.7,86.4 160.5,97.3 124.4,93.3 88.2,107.6 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.3 88.2,192.7 124.4,134.4 160.5,134.4 196.7,125.6 232.9,125.6 269.1,121.5 305.3,115.1 341.5,115.2 377.6,108.7 413.8,110.2 450.0,106.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,111.2 124.4,101.7 160.5,101.9 196.7,93.7 232.9,91.9 269.1,85.3 305.3,84.0 341.5,74.4 377.6,69.6 413.8,55.5 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,108.5 124.4,99.2 160.5,98.2 196.7,90.9 232.9,88.0 269.1,77.4 305.3,69.0 341.5,62.0 377.6,53.8 413.8,43.0 450.0,35.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,97.1 160.5,93.4 196.7,84.3 232.9,83.3 269.1,72.9 305.3,61.9 341.5,54.2 377.6,46.6 413.8,34.5 450.0,25.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,107.6 124.4,93.3 160.5,97.3 196.7,86.4 232.9,79.4 269.1,67.3 305.3,56.5 341.5,50.0 377.6,38.5 413.8,26.4 450.0,17.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.26 µs | 1.18 µs | 2.69 µs | 2.66 µs | 3.05 µs |
| D38 | 1.26 µs | 2.51 µs | 3 µs | 3.48 µs | 2.59 µs |
| D57 | 2.97 µs | 3.72 µs | 5.17 µs | 5.66 µs | 9.63 µs |
| D76 | 2.98 µs | 3.96 µs | 5.41 µs | 7.76 µs | 5.28 µs |
| D115 | 3.22 µs | 4.76 µs | 8.33 µs | 14 µs | 11.9 µs |
| D153 | 3.04 µs | 5.45 µs | 8.07 µs | 14.8 µs | 23 µs |
| D230 | 2.72 µs | 10.1 µs | 19.3 µs | 32.4 µs | 57.5 µs |
| D307 | 3.26 µs | 7.56 µs | 27.5 µs | 73.4 µs | 112 µs |
| D462 | 2.05 µs | 15.2 µs | 66.6 µs | 134 µs | 185 µs |
| D616 | 3.08 µs | 23.2 µs | 122 µs | 242 µs | 454 µs |
| D924 | 2 µs | 69.7 µs | 268 µs | 581 µs | 1.13 ms |
| D1232 | 2.14 µs | 130 µs | 431 µs | 1.14 ms | 2.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.0 88.2,204.9 124.4,186.4 160.5,186.3 196.7,184.6 232.9,185.9 269.1,188.3 305.3,184.3 341.5,194.4 377.6,185.6 413.8,195.0 450.0,193.5 450.0,41.5 413.8,57.4 377.6,77.2 341.5,96.6 305.3,107.6 269.1,122.0 232.9,141.9 196.7,156.2 160.5,173.9 124.4,160.8 88.2,189.3 52.0,185.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.0 88.2,204.9 124.4,186.4 160.5,186.3 196.7,184.6 232.9,185.9 269.1,188.3 305.3,184.3 341.5,194.4 377.6,185.6 413.8,195.0 450.0,193.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,206.3 88.2,190.0 124.4,181.5 160.5,180.1 196.7,176.1 232.9,173.2 269.1,159.7 305.3,166.1 341.5,150.9 377.6,141.7 413.8,117.8 450.0,104.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.5 88.2,186.1 124.4,174.3 160.5,173.3 196.7,164.0 232.9,164.7 269.1,145.7 305.3,138.0 341.5,118.8 377.6,105.6 413.8,88.6 450.0,78.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.7 88.2,182.9 124.4,172.4 160.5,165.5 196.7,152.6 232.9,151.4 269.1,134.5 305.3,116.7 341.5,103.6 377.6,90.8 413.8,71.8 450.0,57.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.8 88.2,189.3 124.4,160.8 160.5,173.9 196.7,156.2 232.9,141.9 269.1,122.0 305.3,107.6 341.5,96.6 377.6,77.2 413.8,57.4 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.73 µs | 3.63 µs | 3.58 µs | 4.13 µs |
| D38 | 1.55 µs | 3.3 µs | 3.91 µs | 4.39 µs | 3.46 µs |
| D57 | 5.64 µs | 5.96 µs | 7.55 µs | 8.08 µs | 11 µs |
| D76 | 5.69 µs | 5.83 µs | 7.67 µs | 11 µs | 7.37 µs |
| D115 | 12.6 µs | 12.2 µs | 10 µs | 21.9 µs | 17.9 µs |
| D153 | 5.96 µs | 7.89 µs | 12.7 µs | 19.1 µs | 28.2 µs |
| D230 | 5.13 µs | 14 µs | 24.5 µs | 38.1 µs | 67.7 µs |
| D307 | 6.57 µs | 11 µs | 53.3 µs | 85.5 µs | 118 µs |
| D462 | 6.63 µs | 23.1 µs | 86.3 µs | 157 µs | 199 µs |
| D616 | 6.16 µs | 27.9 µs | 134 µs | 262 µs | 449 µs |
| D924 | 4.03 µs | 82.5 µs | 290 µs | 562 µs | 994 µs |
| D1232 | 4.27 µs | 144 µs | 418 µs | 1 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.2 88.2,200.4 124.4,172.4 160.5,172.2 196.7,155.0 232.9,171.2 269.1,174.5 305.3,169.1 341.5,168.9 377.6,170.5 413.8,179.8 450.0,178.5 450.0,37.3 413.8,60.1 377.6,77.4 341.5,95.1 305.3,106.4 269.1,118.5 232.9,137.5 196.7,147.3 160.5,166.6 124.4,157.9 88.2,183.1 52.0,179.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.2 88.2,200.4 124.4,172.4 160.5,172.2 196.7,155.0 232.9,171.2 269.1,174.5 305.3,169.1 341.5,168.9 377.6,170.5 413.8,179.8 450.0,178.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.1 88.2,184.1 124.4,171.2 160.5,171.7 196.7,155.6 232.9,165.1 269.1,152.7 305.3,157.9 341.5,141.9 377.6,137.7 413.8,114.2 450.0,102.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,180.4 124.4,166.1 160.5,165.8 196.7,160.0 232.9,154.8 269.1,140.5 305.3,123.7 341.5,113.2 377.6,103.6 413.8,86.9 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.3 88.2,177.9 124.4,164.6 160.5,157.8 196.7,143.0 232.9,145.9 269.1,131.0 305.3,113.4 341.5,100.2 377.6,89.1 413.8,72.5 450.0,60.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,183.1 124.4,157.9 160.5,166.6 196.7,147.3 232.9,137.5 269.1,118.5 305.3,106.4 341.5,95.1 377.6,77.4 413.8,60.1 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.35 µs | 1.07 µs | 2.52 µs | 2.61 µs | 3.01 µs |
| D38 | 1.35 µs | 2.35 µs | 2.85 µs | 3.31 µs | 2.57 µs |
| D57 | 3.1 µs | 3.69 µs | 4.86 µs | 5.51 µs | 9.64 µs |
| D76 | 3.26 µs | 3.46 µs | 5.13 µs | 7.73 µs | 5.21 µs |
| D115 | 3.31 µs | 4.44 µs | 8.36 µs | 14 µs | 11.6 µs |
| D153 | 3.17 µs | 5.18 µs | 8.16 µs | 14.5 µs | 22.8 µs |
| D230 | 2.82 µs | 10.4 µs | 19.6 µs | 31.6 µs | 56.1 µs |
| D307 | 3.38 µs | 7.26 µs | 26.2 µs | 70.6 µs | 108 µs |
| D462 | 2.14 µs | 15.6 µs | 62.6 µs | 132 µs | 177 µs |
| D616 | 3.21 µs | 22.5 µs | 120 µs | 238 µs | 452 µs |
| D924 | 2.16 µs | 67.1 µs | 269 µs | 576 µs | 1.12 ms |
| D1232 | 2.23 µs | 128 µs | 428 µs | 1.13 ms | 2.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.6 88.2,203.5 124.4,185.5 160.5,184.4 196.7,184.0 232.9,184.9 269.1,187.5 305.3,183.5 341.5,193.5 377.6,184.7 413.8,193.3 450.0,192.5 450.0,41.5 413.8,57.5 377.6,77.2 341.5,97.6 305.3,108.3 269.1,122.6 232.9,142.1 196.7,156.7 160.5,174.2 124.4,160.8 88.2,189.5 52.0,186.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.6 88.2,203.5 124.4,185.5 160.5,184.4 196.7,184.0 232.9,184.9 269.1,187.5 305.3,183.5 341.5,193.5 377.6,184.7 413.8,193.3 450.0,192.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.6 88.2,191.5 124.4,181.6 160.5,183.0 196.7,177.6 232.9,174.3 269.1,159.2 305.3,167.0 341.5,150.3 377.6,142.4 413.8,118.7 450.0,104.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.9 88.2,187.3 124.4,175.7 160.5,174.5 196.7,163.9 232.9,164.4 269.1,145.4 305.3,139.1 341.5,120.2 377.6,106.1 413.8,88.5 450.0,78.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.2 88.2,184.0 124.4,172.9 160.5,165.6 196.7,152.7 232.9,151.9 269.1,135.0 305.3,117.5 341.5,104.0 377.6,91.2 413.8,72.0 450.0,57.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.1 88.2,189.5 124.4,160.8 160.5,174.2 196.7,156.7 232.9,142.1 269.1,122.6 305.3,108.3 341.5,97.6 377.6,77.2 413.8,57.5 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.74 µs | 3.65 µs | 3.6 µs | 4.14 µs |
| D38 | 1.56 µs | 3.31 µs | 3.92 µs | 4.4 µs | 3.46 µs |
| D57 | 5.65 µs | 6.01 µs | 7.58 µs | 8.11 µs | 11 µs |
| D76 | 5.72 µs | 5.86 µs | 7.7 µs | 11.1 µs | 7.34 µs |
| D115 | 12.7 µs | 12.2 µs | 9.74 µs | 22.2 µs | 16.7 µs |
| D153 | 6.07 µs | 7.94 µs | 12.5 µs | 18.7 µs | 28 µs |
| D230 | 5.16 µs | 14.1 µs | 24.6 µs | 38.1 µs | 67.9 µs |
| D307 | 6.61 µs | 10.8 µs | 52.3 µs | 85.8 µs | 119 µs |
| D462 | 6.65 µs | 23.1 µs | 87.7 µs | 157 µs | 201 µs |
| D616 | 6.29 µs | 28.3 µs | 134 µs | 263 µs | 447 µs |
| D924 | 4.11 µs | 82.7 µs | 291 µs | 567 µs | 994 µs |
| D1232 | 4.43 µs | 145 µs | 417 µs | 1 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.2 88.2,200.4 124.4,172.4 160.5,172.1 196.7,154.9 232.9,170.8 269.1,174.4 305.3,169.0 341.5,168.9 377.6,170.1 413.8,179.3 450.0,177.7 450.0,37.3 413.8,60.1 377.6,77.5 341.5,94.8 305.3,106.3 269.1,118.4 232.9,137.7 196.7,148.8 160.5,166.7 124.4,157.9 88.2,183.0 52.0,179.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.2 88.2,200.4 124.4,172.4 160.5,172.1 196.7,154.9 232.9,170.8 269.1,174.4 305.3,169.0 341.5,168.9 377.6,170.1 413.8,179.3 450.0,177.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.0 88.2,184.0 124.4,171.1 160.5,171.6 196.7,155.7 232.9,165.0 269.1,152.6 305.3,158.4 341.5,141.9 377.6,137.4 413.8,114.1 450.0,102.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.9 88.2,180.3 124.4,166.0 160.5,165.7 196.7,160.6 232.9,155.2 269.1,140.5 305.3,124.1 341.5,112.8 377.6,103.6 413.8,86.8 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.2 88.2,177.8 124.4,164.5 160.5,157.8 196.7,142.7 232.9,146.4 269.1,131.0 305.3,113.3 341.5,100.2 377.6,89.0 413.8,72.3 450.0,60.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,183.0 124.4,157.9 160.5,166.7 196.7,148.8 232.9,137.7 269.1,118.4 305.3,106.3 341.5,94.8 377.6,77.5 413.8,60.1 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.48 µs | 2.12 µs | 4.28 µs | 4.28 µs | 4.94 µs |
| D38 | 2.5 µs | 3.98 µs | 4.78 µs | 5.54 µs | 4.6 µs |
| D57 | 4.24 µs | 4.76 µs | 6.48 µs | 7.33 µs | 10 µs |
| D76 | 4.06 µs | 4.59 µs | 6.9 µs | 9.86 µs | 6.42 µs |
| D115 | 4.32 µs | 5.89 µs | 10.2 µs | 16.9 µs | 14 µs |
| D153 | 4.07 µs | 6.96 µs | 10.2 µs | 18.7 µs | 26.3 µs |
| D230 | 3.61 µs | 12.8 µs | 23.6 µs | 36.3 µs | 63.9 µs |
| D307 | 4.37 µs | 10.8 µs | 30.9 µs | 79.7 µs | 120 µs |
| D462 | 2.67 µs | 18.6 µs | 71.6 µs | 146 µs | 194 µs |
| D616 | 4.15 µs | 26.4 µs | 133 µs | 260 µs | 491 µs |
| D924 | 2.7 µs | 77.2 µs | 293 µs | 617 µs | 1.19 ms |
| D1232 | 2.82 µs | 142 µs | 464 µs | 1.21 ms | 2.46 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.3 88.2,190.1 124.4,178.6 160.5,179.6 196.7,178.2 232.9,179.5 269.1,182.1 305.3,178.0 341.5,188.7 377.6,179.1 413.8,188.4 450.0,187.4 450.0,40.4 413.8,56.1 377.6,75.5 341.5,95.6 305.3,106.0 269.1,119.7 232.9,139.0 196.7,152.7 160.5,169.6 124.4,159.9 88.2,176.9 52.0,175.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.3 88.2,190.1 124.4,178.6 160.5,179.6 196.7,178.2 232.9,179.5 269.1,182.1 305.3,178.0 341.5,188.7 377.6,179.1 413.8,188.4 450.0,187.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,193.6 88.2,180.0 124.4,176.1 160.5,176.9 196.7,171.5 232.9,167.9 269.1,154.6 305.3,158.4 341.5,146.5 377.6,138.9 413.8,115.6 450.0,102.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,176.0 124.4,169.4 160.5,168.0 196.7,159.6 232.9,159.5 269.1,141.4 305.3,135.5 341.5,117.3 377.6,103.7 413.8,86.7 450.0,76.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,172.8 124.4,166.7 160.5,160.3 196.7,148.7 232.9,146.4 269.1,132.0 305.3,114.9 341.5,101.8 377.6,89.2 413.8,70.5 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.3 88.2,176.9 124.4,159.9 160.5,169.6 196.7,152.7 232.9,139.0 269.1,119.7 305.3,106.0 341.5,95.6 377.6,75.5 413.8,56.1 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.66 µs | 1.76 µs | 3.81 µs | 3.76 µs | 4.34 µs |
| D38 | 1.65 µs | 3.47 µs | 3.95 µs | 4.42 µs | 3.49 µs |
| D57 | 5.81 µs | 6.16 µs | 7.88 µs | 8.45 µs | 11.4 µs |
| D76 | 5.8 µs | 5.97 µs | 8.07 µs | 11.5 µs | 7.6 µs |
| D115 | 13 µs | 12.8 µs | 10.4 µs | 22.6 µs | 17.4 µs |
| D153 | 6.05 µs | 8.39 µs | 13.2 µs | 20.2 µs | 29.1 µs |
| D230 | 5.29 µs | 15.3 µs | 25.3 µs | 38.8 µs | 69.1 µs |
| D307 | 6.71 µs | 11 µs | 54.5 µs | 86.6 µs | 120 µs |
| D462 | 6.83 µs | 23.8 µs | 88.1 µs | 159 µs | 199 µs |
| D616 | 6.51 µs | 28.9 µs | 136 µs | 266 µs | 452 µs |
| D924 | 4.29 µs | 84.3 µs | 293 µs | 571 µs | 1 ms |
| D1232 | 5.16 µs | 147 µs | 422 µs | 1.01 ms | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.0 88.2,199.1 124.4,171.8 160.5,171.8 196.7,154.2 232.9,170.9 269.1,173.8 305.3,168.7 341.5,168.3 377.6,169.3 413.8,178.4 450.0,174.4 450.0,37.2 413.8,59.9 377.6,77.2 341.5,95.1 305.3,106.0 269.1,118.0 232.9,136.8 196.7,148.0 160.5,165.9 124.4,157.1 88.2,182.9 52.0,178.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.0 88.2,199.1 124.4,171.8 160.5,171.8 196.7,154.2 232.9,170.9 269.1,173.8 305.3,168.7 341.5,168.3 377.6,169.3 413.8,178.4 450.0,174.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.7 88.2,183.0 124.4,170.5 160.5,171.2 196.7,154.7 232.9,163.8 269.1,150.8 305.3,157.9 341.5,141.1 377.6,136.9 413.8,113.7 450.0,101.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.0 88.2,180.1 124.4,165.2 160.5,164.6 196.7,159.2 232.9,153.9 269.1,139.9 305.3,123.2 341.5,112.7 377.6,103.3 413.8,86.7 450.0,78.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.3 88.2,177.7 124.4,163.7 160.5,157.0 196.7,142.3 232.9,144.8 269.1,130.6 305.3,113.1 341.5,100.0 377.6,88.8 413.8,72.2 450.0,59.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.1 88.2,182.9 124.4,157.1 160.5,165.9 196.7,148.0 232.9,136.8 269.1,118.0 305.3,106.0 341.5,95.1 377.6,77.2 413.8,59.9 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 133 ns | 133 ns | 175 ns | 161 ns | 178 ns |
| D38 | 136 ns | 160 ns | 165 ns | 186 ns | 140 ns |
| D57 | 238 ns | 337 ns | 353 ns | 325 ns | 446 ns |
| D76 | 235 ns | 296 ns | 330 ns | 443 ns | 265 ns |
| D115 | 483 ns | 565 ns | 562 ns | 781 ns | 549 ns |
| D153 | 470 ns | 579 ns | 565 ns | 643 ns | 715 ns |
| D230 | 579 ns | 852 ns | 1.11 µs | 1.15 µs | 1.52 µs |
| D307 | 1.02 µs | 890 ns | 1.71 µs | 2.33 µs | 2.65 µs |
| D462 | 1.28 µs | 1.46 µs | 2.36 µs | 3.08 µs | 3.32 µs |
| D616 | 1.31 µs | 1.39 µs | 2.91 µs | 4.02 µs | 6.1 µs |
| D924 | 1.02 µs | 2.83 µs | 4.94 µs | 7.17 µs | 11.3 µs |
| D1232 | 1.45 µs | 4.46 µs | 7.2 µs | 12.1 µs | 31.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.7 88.2,201.1 124.4,184.9 160.5,185.3 196.7,164.4 232.9,165.2 269.1,159.1 305.3,142.6 341.5,136.1 377.6,135.6 413.8,142.7 450.0,132.5 450.0,43.7 413.8,73.0 377.6,91.0 341.5,108.6 305.3,115.1 269.1,131.1 232.9,153.0 196.7,160.7 160.5,181.8 124.4,166.7 88.2,200.3 52.0,193.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.7 88.2,201.1 124.4,184.9 160.5,185.3 196.7,164.4 232.9,165.2 269.1,159.1 305.3,142.6 341.5,136.1 377.6,135.6 413.8,142.7 450.0,132.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,201.8 88.2,196.5 124.4,174.8 160.5,178.6 196.7,159.9 232.9,159.1 269.1,148.0 305.3,146.7 341.5,132.4 377.6,133.8 413.8,113.2 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,193.7 88.2,195.5 124.4,173.4 160.5,175.4 196.7,160.0 232.9,159.9 269.1,140.4 305.3,127.9 341.5,118.4 377.6,112.4 413.8,97.1 450.0,86.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.2 88.2,192.1 124.4,175.9 160.5,166.9 196.7,150.5 232.9,156.1 269.1,139.2 305.3,118.8 341.5,110.8 377.6,103.0 413.8,86.3 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,193.3 88.2,200.3 124.4,166.7 160.5,181.8 196.7,160.7 232.9,153.0 269.1,131.1 305.3,115.1 341.5,108.6 377.6,91.0 413.8,73.0 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 156 ns | 156 ns | 211 ns | 202 ns | 218 ns |
| D38 | 150 ns | 191 ns | 197 ns | 198 ns | 171 ns |
| D57 | 316 ns | 408 ns | 423 ns | 402 ns | 540 ns |
| D76 | 317 ns | 372 ns | 408 ns | 523 ns | 304 ns |
| D115 | 646 ns | 676 ns | 616 ns | 928 ns | 581 ns |
| D153 | 630 ns | 713 ns | 635 ns | 706 ns | 834 ns |
| D230 | 772 ns | 1.01 µs | 1.24 µs | 1.33 µs | 1.67 µs |
| D307 | 1.35 µs | 929 ns | 2 µs | 2.63 µs | 3.03 µs |
| D462 | 1.45 µs | 1.68 µs | 2.67 µs | 3.39 µs | 3.69 µs |
| D616 | 1.65 µs | 1.56 µs | 3.31 µs | 4.42 µs | 6.59 µs |
| D924 | 1.36 µs | 3.29 µs | 5.43 µs | 7.67 µs | 12 µs |
| D1232 | 1.88 µs | 5.23 µs | 7.99 µs | 13.2 µs | 32.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.2 88.2,198.2 124.4,176.7 160.5,176.6 196.7,156.0 232.9,156.7 269.1,150.8 305.3,134.6 341.5,132.5 377.6,128.9 413.8,134.4 450.0,125.0 450.0,42.7 413.8,71.4 377.6,88.7 341.5,105.6 305.3,111.2 269.1,128.5 232.9,148.6 196.7,159.1 160.5,177.8 124.4,161.1 88.2,194.4 52.0,187.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.2 88.2,198.2 124.4,176.7 160.5,176.6 196.7,156.0 232.9,156.7 269.1,150.8 305.3,134.6 341.5,132.5 377.6,128.9 413.8,134.4 450.0,125.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.2 88.2,191.3 124.4,169.3 160.5,171.9 196.7,154.7 232.9,153.1 269.1,143.1 305.3,145.5 341.5,128.3 377.6,130.5 413.8,108.9 450.0,95.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,190.4 124.4,168.2 160.5,169.3 196.7,157.4 232.9,156.5 269.1,137.0 305.3,123.2 341.5,114.9 377.6,108.7 413.8,94.3 450.0,83.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.7 88.2,190.2 124.4,169.7 160.5,162.1 196.7,145.5 232.9,153.4 269.1,135.0 305.3,115.3 341.5,108.0 377.6,100.3 413.8,84.3 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,194.4 124.4,161.1 160.5,177.8 196.7,159.1 232.9,148.6 269.1,128.5 305.3,111.2 341.5,105.6 377.6,88.7 413.8,71.4 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
